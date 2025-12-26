use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Compute Adler-32 hash on `Adler32Hash` type.
///
/// # Arguments
/// * `hash` - A Adler-32 hash-able type.
///
/// # Examples
/// ```rust
/// use simd_adler32::adler32;
///
/// let hash = adler32(b"Adler-32");
/// println!("{}", hash); // 800813569
/// ```
pub fn adler32<H: Adler32Hash>(hash: &H) -> u32 {
    hash.hash()
}
