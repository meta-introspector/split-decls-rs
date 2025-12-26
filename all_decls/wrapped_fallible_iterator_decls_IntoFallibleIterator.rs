use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Conversion into a `FallibleIterator`.
pub trait IntoFallibleIterator {
    /// The elements of the iterator.
    type Item;
    /// The error value of the iterator.
    type Error;
    /// The iterator.
    type IntoFallibleIter: FallibleIterator<Item = Self::Item, Error = Self::Error>;
    /// Creates a fallible iterator from a value.
    fn into_fallible_iter(self) -> Self::IntoFallibleIter;
}
