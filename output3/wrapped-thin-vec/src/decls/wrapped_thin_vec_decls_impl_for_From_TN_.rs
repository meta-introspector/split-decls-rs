use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T, const N: usize> From<[T; N]> for ThinVec<T> {
    /// Allocate a `ThinVec<T>` and move `s`'s items into it.
    ///
    /// # Examples
    ///
    /// ```
    /// use thin_vec::{ThinVec, thin_vec};
    ///
    /// assert_eq!(ThinVec::from([1, 2, 3]), thin_vec![1, 2, 3]);
    /// ```
    fn from(s: [T; N]) -> ThinVec<T> {
        core::iter::IntoIterator::into_iter(s).collect()
    }
}
