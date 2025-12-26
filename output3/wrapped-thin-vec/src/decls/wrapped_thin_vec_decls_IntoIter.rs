use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An iterator that moves out of a vector.
///
/// This `struct` is created by the [`ThinVec::into_iter`][]
/// (provided by the [`IntoIterator`] trait).
///
/// # Example
///
/// ```
/// use thin_vec::thin_vec;
///
/// let v = thin_vec![0, 1, 2];
/// let iter: thin_vec::IntoIter<_> = v.into_iter();
/// ```
pub struct IntoIter<T> {
    vec: ThinVec<T>,
    start: usize,
}
