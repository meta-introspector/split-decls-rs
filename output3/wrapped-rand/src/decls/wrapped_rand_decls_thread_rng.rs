use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Access the thread-local generator
///
/// Use [`rand::rng()`](rng()) instead.
#[cfg(feature = "thread_rng")]
#[deprecated(since = "0.9.0", note = "Renamed to `rng`")]
#[inline]
pub fn thread_rng() -> crate::rngs::ThreadRng {
    rng()
}
