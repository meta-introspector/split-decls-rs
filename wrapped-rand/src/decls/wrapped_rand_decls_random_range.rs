use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Generate a random value in the given range using the thread-local random number generator.
///
/// This function is shorthand for
/// <code>[rng()].[random_range](Rng::random_range)(<var>range</var>)</code>.
///
/// # Example
///
/// ```
/// let y: f32 = rand::random_range(0.0..=1e9);
/// println!("{}", y);
///
/// let words: Vec<&str> = "Mary had a little lamb".split(' ').collect();
/// println!("{}", words[rand::random_range(..words.len())]);
/// ```
/// Note that the first example can also be achieved (without `collect`'ing
/// to a `Vec`) using [`seq::IteratorRandom::choose`].
#[cfg(feature = "thread_rng")]
#[inline]
pub fn random_range<T, R>(range: R) -> T
where
    T: distr::uniform::SampleUniform,
    R: distr::uniform::SampleRange<T>,
{
    rng().random_range(range)
}
