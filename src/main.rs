use std::io::stdin;

fn longest_palindrome(input_str: String) -> String {
    let mut left_index: usize;
    let mut right_index: usize;

    let mut lp_left_index: usize = 0;
    let mut lp_right_index: usize = 0;
    
    if input_str.len() == 0 {
        return input_str;
    }

    for current_index in 0..input_str.len() {
        left_index = current_index;
        right_index = current_index;

        let mut not_odd: bool = false;
        let mut not_even: bool = false;

        let mut odd_palindrome_length: i32 = -1;
        let mut even_palindrome_length: i32 = 0;

        loop {
            if !not_odd && input_str.chars().nth(left_index) == input_str.chars().nth(right_index) {
                odd_palindrome_length += 2;
            } else {
                not_odd = true;
            }
            
            if right_index < input_str.len() - 1 {
                if !not_even {
                    if input_str.chars().nth(left_index) == input_str.chars().nth(right_index + 1) {
                        even_palindrome_length += 2;
                    } else {
                        not_even = true;
                    }
                }
            } else {
                not_even = true;
            }

            if not_odd && not_even {
                left_index += 1;
                right_index -= 1;
                break;
            }
            if left_index == 0 || right_index == input_str.len() - 1 {
                break;
            }

            left_index -= 1;
            right_index += 1;

        }
        right_index += (even_palindrome_length > odd_palindrome_length) as usize;
        if right_index - left_index > lp_right_index - lp_left_index {
            lp_left_index = left_index;
            lp_right_index = right_index;
        }
    }
    
    input_str[lp_left_index..lp_right_index + 1].to_string()
}

fn main() {
    let mut input_line: String = String::new();
    
    stdin()
        .read_line(&mut input_line)
        .expect("Failed to read a line!");

    println!("{}", longest_palindrome(input_line));
}