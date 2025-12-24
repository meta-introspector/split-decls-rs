use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Check a raw string literal for validity
///
/// Takes the contents of a raw string literal (without quotes)
/// and produces a sequence of characters or errors,
/// which are returned by invoking `callback`.
/// NOTE: Does no escaping, but produces errors for bare carriage return ('\r').
pub fn check_raw_str(
    src: &str,
    callback: impl FnMut(Range<usize>, Result<char, EscapeError>),
) {
    str::check_raw(src, callback);
}
