use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Check a raw C string literal for validity
///
/// Takes the contents of a raw C string literal (without quotes)
/// and produces a sequence of characters or errors,
/// which are returned by invoking `callback`.
/// NOTE: Does no escaping, but produces errors for bare carriage return ('\r').
pub fn check_raw_c_str(
    src: &str,
    callback: impl FnMut(Range<usize>, Result<NonZero<char>, EscapeError>),
) {
    CStr::check_raw(src, callback);
}
