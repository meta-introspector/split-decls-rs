use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Calculates the Adler-32 checksum of a byte slice.
///
/// This is a convenience function around the [`Adler32`] type.
///
/// [`Adler32`]: struct.Adler32.html
pub fn adler32_slice(data: &[u8]) -> u32 {
    let mut h = Adler32::new();
    h.write_slice(data);
    h.checksum()
}
