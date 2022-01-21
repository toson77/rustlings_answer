// quiz2.rs
// This is a quiz for the following sections:
// - Strings

// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue"); // blue
    string("red".to_string()); //red
    string(String::from("hi")); // hi
    string("rust is fun!".to_owned()); // rust is fun!
    string("nice weather".into()); // nice weather
    string(format!("Interpolation {}", "Station")); // Interpolation Station
    string_slice(&String::from("abc")[0..1]); // a
    string_slice("  hello there ".trim()); //hello there
    string("Happy Monday!".to_string().replace("Mon", "Tues")); // Happy Tuesday!
    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // my shift key is sticky
}
