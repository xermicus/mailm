#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate fast_chemail;

use std::io::Read;
use std::io::Write;
use std::fs::File;
use rocket::response::content;
use fast_chemail::is_valid_email;
use rocket::http::uri::URI;


const LIST_FILE: &'static str= "newspeople.txt";


#[derive(FromForm)]
struct EmailAddress<'r> {
	addr: &'r str,
}


fn render(news_list: String) -> content::HTML<String> {
	let mut formated_list = String::new();
	for s in news_list.split(' ').skip(1) {
		formated_list.push_str(s);
		formated_list.push_str("<br />");
	}

	let content = format!("<html><body>
				<p><b>Add or Remove a receipent:</b></p>
                                <form action=\"/\" method=\"get\">
                                EMail: <input type=\"text\" name=\"addr\"> 
                                <input type=\"submit\" value=\"doit!\"><br />
                                <p><b>Current subscribers:</b></p><p> {} </p>
                         	</body></html>", formated_list);

	content::HTML(String::from(content))
}


fn read_file(path: &str) -> String {
	let mut f = File::open(path)
	 .expect("file not found");
	let mut result = String::new();
	f.read_to_string(&mut result) 
	 .expect("error reading file");
	result
}


fn write_file(path: &str, new_list: &mut String) {
	while new_list.chars().last() == Some(' ') ||
	 new_list.chars().last() == Some(',') {
		new_list.pop();
	}
	if new_list.chars().last() != Some('\x0a') {
		new_list.push('\x0a');
	}

	let mut f = File::create(path)
	 .expect("Error: File not found");
	f.write_all(new_list.as_bytes())
	 .expect("Error: Can not write to file");
}


#[get("/")]
fn index() -> content::HTML<String> {
	render(read_file(LIST_FILE))
}


#[get("/?<doit>")]
fn process(doit: EmailAddress) -> content::HTML<String> {
	let uri = URI::new(doit.addr);
	let mail_address = URI::percent_decode(uri.path().as_bytes())
	 .expect("decoded")
	 .to_string();
	
	if !is_valid_email(&mail_address) {
		return content::HTML(String::from("Error: Invalid EMail address. Aborting."));
	}

	let mut old_list = read_file(LIST_FILE);
	let mut new_list = String::new();
	let in_list = old_list.contains(&mail_address);
	
	if in_list {
		for s in old_list.split(' ').skip(1) {
			if !s.contains(&mail_address) {
				new_list.push_str(s);
				new_list.push(' ');
			}
		}
		new_list.insert_str(0, "newsletter: ");
	} else {
		old_list.pop();
		new_list.push_str(&old_list);
		new_list.push_str(", ");
		new_list.push_str(&mail_address);
	}

	write_file(LIST_FILE, &mut new_list);

	render(read_file(LIST_FILE))
}


fn main() {
	rocket::ignite().mount("/", routes![index, process]).launch()
}

