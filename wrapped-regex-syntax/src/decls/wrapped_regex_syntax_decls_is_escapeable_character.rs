use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Returns true if the given character can be escaped in a regex.
///
/// This returns true in all cases that `is_meta_character` returns true, but
/// also returns true in some cases where `is_meta_character` returns false.
/// For example, `%` is not a meta character, but it is escapable. That is,
/// `%` and `\%` both match a literal `%` in all contexts.
///
/// The purpose of this routine is to provide knowledge about what characters
/// may be escaped. Namely, most regex engines permit "superfluous" escapes
/// where characters without any special significance may be escaped even
/// though there is no actual _need_ to do so.
///
/// This will return false for some characters. For example, `e` is not
/// escapable. Therefore, `\e` will either result in a parse error (which is
/// true today), or it could backwards compatibly evolve into a new construct
/// with its own meaning. Indeed, that is the purpose of banning _some_
/// superfluous escapes: it provides a way to evolve the syntax in a compatible
/// manner.
///
/// # Example
///
/// ```
/// use regex_syntax::is_escapeable_character;
///
/// assert!(is_escapeable_character('?'));
/// assert!(is_escapeable_character('-'));
/// assert!(is_escapeable_character('&'));
/// assert!(is_escapeable_character('#'));
/// assert!(is_escapeable_character('%'));
/// assert!(is_escapeable_character('/'));
/// assert!(is_escapeable_character('!'));
/// assert!(is_escapeable_character('"'));
///
/// assert!(!is_escapeable_character('e'));
/// ```
pub fn is_escapeable_character(c: char) -> bool {
    if is_meta_character(c) {
        return true;
    }
    if !c.is_ascii() {
        return false;
    }
    match c {
        '0'..='9' | 'A'..='Z' | 'a'..='z' => false,
        '<' | '>' => false,
        _ => true,
    }
}
