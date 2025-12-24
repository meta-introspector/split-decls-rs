use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(feature = "equivalent"))]
/// Key equivalence trait.
///
/// This trait defines the function used to compare the input value with the
/// map keys (or set values) during a lookup operation such as [`HashMap::get`]
/// or [`HashSet::contains`].
/// It is provided with a blanket implementation based on the
/// [`Borrow`](core::borrow::Borrow) trait.
///
/// # Correctness
///
/// Equivalent values must hash to the same value.
pub trait Equivalent<K: ?Sized> {
    /// Checks if this value is equivalent to the given key.
    ///
    /// Returns `true` if both values are equivalent, and `false` otherwise.
    ///
    /// # Correctness
    ///
    /// When this function returns `true`, both `self` and `key` must hash to
    /// the same value.
    fn equivalent(&self, key: &K) -> bool;
}
