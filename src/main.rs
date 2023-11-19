use std::io;

fn main() {
    let mut input = String::new();

    println!("Please input your text");

    match io::stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(err) => println!("ERR: {err}"),
    }

    let pig_text: String = input.split_whitespace().map(to_pig_latin).collect();

    println!("{pig_text}")
}

fn to_pig_latin(str: &str) -> String {
    let mut it = str.chars();
    let first_char = it.next().unwrap();

    match first_char.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay ", str),
        'a'..='z' => format!("{}-{}ay ", it.collect::<String>(), first_char),
        _ => str.to_string(),
    }
}
