use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Given an original string like `AACC`, and a suggestion like `AABBCC`, try to detect
/// the case where a substring of the suggestion is "sandwiched" in the original, like
/// `BB` is. Return the length of the prefix, the "trimmed" suggestion, and the length
/// of the suffix.
fn as_substr<'a>(original: &'a str, suggestion: &'a str) -> Option<(usize, &'a str, usize)> {
    let common_prefix = original
        .chars()
        .zip(suggestion.chars())
        .take_while(|(c1, c2)| c1 == c2)
        .map(|(c, _)| c.len_utf8())
        .sum();
    let original = &original[common_prefix..];
    let suggestion = &suggestion[common_prefix..];
    if suggestion.ends_with(original) {
        let common_suffix = original.len();
        Some((
            common_prefix,
            &suggestion[..suggestion.len() - original.len()],
            common_suffix,
        ))
    } else {
        None
    }
}
