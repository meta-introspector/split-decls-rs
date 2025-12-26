use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Fill any type implementing [`Fill`] with random data
///
/// This function is shorthand for
/// <code>[rng()].[fill](Rng::fill)(<var>dest</var>)</code>.
///
/// # Example
///
/// ```
/// let mut arr = [0i8; 20];
/// rand::fill(&mut arr[..]);
/// ```
///
/// Note that you can instead use [`random()`] to generate an array of random
/// data, though this is slower for small elements (smaller than the RNG word
/// size).
#[cfg(feature = "thread_rng")]
#[inline]
#[track_caller]
pub fn fill<T: Fill>(dest: &mut [T]) {
    Fill::fill_slice(dest, &mut rng())
}
