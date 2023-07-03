// https://google.github.io/comprehensive-rust/exercises/day-2/luhn.html

// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
    let cc_number = cc_number.replace(' ', "");
    if cc_number.len() < 2 {
        return false;
    }
    let mut sum = 0;
    for (i, c) in cc_number.chars().rev().enumerate() {
        if !c.is_ascii_digit() {
            return false;
        }
        match c.to_digit(10) {
            Some(digit) => {
                sum += if i % 2 == 0 {
                    digit
                } else {
                    let dd = digit * 2;
                    // Sum the digits of dd, which is guaranteed to be between 0 and 18.
                    // Divide by 10 to get the first digit, mod by 10 to get the second digit
                    dd / 10 + dd % 10
                }
            },
            None => return false,
        }
    }
    sum % 10 == 0
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {}
