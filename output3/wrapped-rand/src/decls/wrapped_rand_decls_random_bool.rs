use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Return a bool with a probability `p` of being true.
///
/// This function is shorthand for
/// <code>[rng()].[random_bool](Rng::random_bool)(<var>p</var>)</code>.
///
/// # Example
///
/// ```
/// println!("{}", rand::random_bool(1.0 / 3.0));
/// ```
///
/// # Panics
///
/// If `p < 0` or `p > 1`.
#[cfg(feature = "thread_rng")]
#[inline]
#[track_caller]
pub fn random_bool(p: f64) -> bool {
    rng().random_bool(p)
}
