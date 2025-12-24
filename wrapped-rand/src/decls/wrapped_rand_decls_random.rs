use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Generate a random value using the thread-local random number generator.
///
/// This function is shorthand for <code>[rng()].[random()](Rng::random)</code>:
///
/// -   See [`ThreadRng`] for documentation of the generator and security
/// -   See [`StandardUniform`] for documentation of supported types and distributions
///
/// # Examples
///
/// ```
/// let x = rand::random::<u8>();
/// println!("{}", x);
///
/// let y = rand::random::<f64>();
/// println!("{}", y);
///
/// if rand::random() { // generates a boolean
///     println!("Better lucky than good!");
/// }
/// ```
///
/// If you're calling `random()` repeatedly, consider using a local `rng`
/// handle to save an initialization-check on each usage:
///
/// ```
/// use rand::Rng; // provides the `random` method
///
/// let mut rng = rand::rng(); // a local handle to the generator
///
/// let mut v = vec![1, 2, 3];
///
/// for x in v.iter_mut() {
///     *x = rng.random();
/// }
/// ```
///
/// [`StandardUniform`]: distr::StandardUniform
/// [`ThreadRng`]: rngs::ThreadRng
#[cfg(feature = "thread_rng")]
#[inline]
pub fn random<T>() -> T
where
    StandardUniform: Distribution<T>,
{
    rng().random()
}
