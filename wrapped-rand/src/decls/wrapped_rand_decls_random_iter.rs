use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Return an iterator over [`random()`] variates
///
/// This function is shorthand for
/// <code>[rng()].[random_iter](Rng::random_iter)()</code>.
///
/// # Example
///
/// ```
/// let v: Vec<i32> = rand::random_iter().take(5).collect();
/// println!("{v:?}");
/// ```
#[cfg(feature = "thread_rng")]
#[inline]
pub fn random_iter<T>() -> distr::Iter<StandardUniform, rngs::ThreadRng, T>
where
    StandardUniform: Distribution<T>,
{
    rng().random_iter()
}
