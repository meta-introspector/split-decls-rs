use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Unescape a byte literal
///
/// Takes the contents of a byte literal (without quotes),
/// and returns an unescaped byte or an error.
#[inline]
pub fn unescape_byte(src: &str) -> Result<u8, EscapeError> {
    <[u8]>::unescape_single(&mut src.chars())
}
