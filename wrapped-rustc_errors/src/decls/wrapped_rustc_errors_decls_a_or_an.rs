use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Grammatical tool for displaying messages to end users in a nice form.
///
/// Returns "an" if the given string starts with a vowel, and "a" otherwise.
pub fn a_or_an(s: &str) -> &'static str {
    let mut chars = s.chars();
    let Some(mut first_alpha_char) = chars.next() else {
        return "a";
    };
    if first_alpha_char == '`' {
        let Some(next) = chars.next() else {
            return "a";
        };
        first_alpha_char = next;
    }
    if ["a", "e", "i", "o", "u", "&"].contains(&&first_alpha_char.to_lowercase().to_string()[..]) {
        "an"
    } else {
        "a"
    }
}
