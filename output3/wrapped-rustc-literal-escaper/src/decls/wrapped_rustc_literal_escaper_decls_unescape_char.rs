use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Unescape a char literal
///
/// Takes the contents of a char literal (without quotes),
/// and returns an unescaped char or an error.
#[inline]
pub fn unescape_char(src: &str) -> Result<char, EscapeError> {
    str::unescape_single(&mut src.chars())
}
