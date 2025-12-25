use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Unescape a string literal
///
/// Takes the contents of a string literal (without quotes)
/// and produces a sequence of escaped characters or errors,
/// which are returned by invoking `callback`.
pub fn unescape_str(src: &str, callback: impl FnMut(Range<usize>, Result<char, EscapeError>)) {
    str::unescape(src, callback)
}
