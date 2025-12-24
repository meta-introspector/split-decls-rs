use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Returns true if and only if the given character is an ASCII word character.
///
/// An ASCII word character is defined by the following character class:
/// `[_0-9a-zA-Z]`.
pub fn is_word_byte(c: u8) -> bool {
    match c {
        b'_' | b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z' => true,
        _ => false,
    }
}
