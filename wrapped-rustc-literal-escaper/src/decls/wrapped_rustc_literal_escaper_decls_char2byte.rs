use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Turn an ascii char into a byte
#[inline]
fn char2byte(c: char) -> Result<u8, EscapeError> {
    if c.is_ascii() { Ok(c as u8) } else { Err(EscapeError::NonAsciiCharInByte) }
}
