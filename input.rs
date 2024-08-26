#![allow(warnings)]

use std::io::{self, read_to_string};

fn string_to_vector_string(s: String) -> Vec<String> {

    let mut ans: Vec<String> = Vec::new();
    let mut buf: String = String::new();
    let mut last_char_is_escape: bool = false;
    let mut under_single_quotes: bool = false;
    let mut under_double_quotes: bool = false;
    for c in s.as_bytes() {
        print!("{},", *c as char);
        match c {
            b'\'' => {
                if last_char_is_escape {
                    buf.push('\'');
                    last_char_is_escape = false;
                } else {
                    if under_single_quotes {
                        under_single_quotes = false;
                        ans.push(buf.clone());
                        buf.clear();
                    } else {
                        if under_double_quotes {
                            buf.push('\'');
                        } else {
                            under_single_quotes = true;
                            if buf != String::from("") {
                                ans.push(buf.clone());
                            }
                            buf.clear();
                        }
                    }
                }
            },
            b'\"' => {
                if last_char_is_escape {
                    buf.push('\"');
                    last_char_is_escape = false;
                } else {
                    if under_double_quotes {
                        under_double_quotes = false;
                        ans.push(buf.clone());
                        buf.clear();
                    } else {
                        if under_single_quotes {
                            buf.push('\"');
                        } else {
                            under_double_quotes = true;
                            if buf != String::from("") {
                                ans.push(buf.clone());
                            }
                            buf.clear();
                        }
                    }
                }
            },
            b'\\' => {
                if last_char_is_escape {
                    buf.push('\\');
                    last_char_is_escape = false;
                } else {
                    last_char_is_escape = true;
                }
            },
            b' ' => {
                if last_char_is_escape || under_double_quotes || under_single_quotes {
                    buf.push(' ');
                    last_char_is_escape = false;
                }
                else {
                    ans.push(buf.clone());
                    buf.clear();
                }
            },
            any_char => {
                if last_char_is_escape {
                    last_char_is_escape = false;
                }
                buf.push(*c as char);
            },
        }
    }
    ans.push(buf);
    if(ans[ans.len()-1] == String::from("")){
        ans.pop();
    }
    return ans;

}

fn main() {

    let mut s = String::from("git commit -m \"this is a github message\"");
    let x = string_to_vector_string(s);
    println!("Hello, world! {:#?}",x);
}
