use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Check a raw byte string literal for validity
///
/// Takes the contents of a raw byte string literal (without quotes)
/// and produces a sequence of bytes or errors,
/// which are returned by invoking `callback`.
/// NOTE: Does no escaping, but produces errors for bare carriage return ('\r').
pub fn check_raw_byte_str(src: &str, callback: impl FnMut(Range<usize>, Result<u8, EscapeError>)) {
    <[u8]>::check_raw(src, callback);
}
