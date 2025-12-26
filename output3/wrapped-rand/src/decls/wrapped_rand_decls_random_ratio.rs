use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Return a bool with a probability of `numerator/denominator` of being
/// true.
///
/// That is, `random_ratio(2, 3)` has chance of 2 in 3, or about 67%, of
/// returning true. If `numerator == denominator`, then the returned value
/// is guaranteed to be `true`. If `numerator == 0`, then the returned
/// value is guaranteed to be `false`.
///
/// See also the [`Bernoulli`] distribution, which may be faster if
/// sampling from the same `numerator` and `denominator` repeatedly.
///
/// This function is shorthand for
/// <code>[rng()].[random_ratio](Rng::random_ratio)(<var>numerator</var>, <var>denominator</var>)</code>.
///
/// # Panics
///
/// If `denominator == 0` or `numerator > denominator`.
///
/// # Example
///
/// ```
/// println!("{}", rand::random_ratio(2, 3));
/// ```
///
/// [`Bernoulli`]: distr::Bernoulli
#[cfg(feature = "thread_rng")]
#[inline]
#[track_caller]
pub fn random_ratio(numerator: u32, denominator: u32) -> bool {
    rng().random_ratio(numerator, denominator)
}
