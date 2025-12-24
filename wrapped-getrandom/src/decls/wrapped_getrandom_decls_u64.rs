use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Get random `u64` from the system's preferred random number source.
///
/// # Examples
///
/// ```
/// # fn main() -> Result<(), getrandom::Error> {
/// let rng_seed = getrandom::u64()?;
/// # Ok(()) }
/// ```
#[inline]
pub fn u64() -> Result<u64, Error> {
    backends::inner_u64()
}
