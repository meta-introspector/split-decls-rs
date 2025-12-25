use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Unescape a byte string literal
///
/// Takes the contents of a byte string literal (without quotes)
/// and produces a sequence of escaped bytes or errors,
/// which are returned by invoking `callback`.
pub fn unescape_byte_str(
    src: &str,
    callback: impl FnMut(Range<usize>, Result<u8, EscapeError>),
) {
    <[u8]>::unescape(src, callback)
}
