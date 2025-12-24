use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Returns true if the given character has significance in a regex.
///
/// Generally speaking, these are the only characters which _must_ be escaped
/// in order to match their literal meaning. For example, to match a literal
/// `|`, one could write `\|`. Sometimes escaping isn't always necessary. For
/// example, `-` is treated as a meta character because of its significance
/// for writing ranges inside of character classes, but the regex `-` will
/// match a literal `-` because `-` has no special meaning outside of character
/// classes.
///
/// In order to determine whether a character may be escaped at all, the
/// [`is_escapeable_character`] routine should be used. The difference between
/// `is_meta_character` and `is_escapeable_character` is that the latter will
/// return true for some characters that are _not_ meta characters. For
/// example, `%` and `\%` both match a literal `%` in all contexts. In other
/// words, `is_escapeable_character` includes "superfluous" escapes.
///
/// Note that the set of characters for which this function returns `true` or
/// `false` is fixed and won't change in a semver compatible release. (In this
/// case, "semver compatible release" actually refers to the `regex` crate
/// itself, since reducing or expanding the set of meta characters would be a
/// breaking change for not just `regex-syntax` but also `regex` itself.)
///
/// # Example
///
/// ```
/// use regex_syntax::is_meta_character;
///
/// assert!(is_meta_character('?'));
/// assert!(is_meta_character('-'));
/// assert!(is_meta_character('&'));
/// assert!(is_meta_character('#'));
///
/// assert!(!is_meta_character('%'));
/// assert!(!is_meta_character('/'));
/// assert!(!is_meta_character('!'));
/// assert!(!is_meta_character('"'));
/// assert!(!is_meta_character('e'));
/// ```
pub fn is_meta_character(c: char) -> bool {
    match c {
        '\\' | '.' | '+' | '*' | '?' | '(' | ')' | '|' | '[' | ']' | '{' | '}' | '^'
        | '$' | '#' | '&' | '-' | '~' => true,
        _ => false,
    }
}
