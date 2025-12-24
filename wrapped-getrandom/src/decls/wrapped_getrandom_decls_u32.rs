use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Get random `u32` from the system's preferred random number source.
///
/// # Examples
///
/// ```
/// # fn main() -> Result<(), getrandom::Error> {
/// let rng_seed = getrandom::u32()?;
/// # Ok(()) }
/// ```
#[inline]
pub fn u32() -> Result<u32, Error> {
    backends::inner_u32()
}
