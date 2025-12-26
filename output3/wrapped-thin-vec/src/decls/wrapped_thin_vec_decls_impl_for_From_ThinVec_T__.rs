use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T> From<ThinVec<T>> for Box<[T]> {
    /// Convert a vector into a boxed slice.
    ///
    /// If `v` has excess capacity, its items will be moved into a
    /// newly-allocated buffer with exactly the right capacity.
    ///
    /// **NOTE:** unlike `std`, this must reallocate to change the layout!
    ///
    /// # Examples
    ///
    /// ```
    /// use thin_vec::{ThinVec, thin_vec};
    /// assert_eq!(Box::from(thin_vec![1, 2, 3]), thin_vec![1, 2, 3].into_iter().collect());
    /// ```
    fn from(v: ThinVec<T>) -> Self {
        v.into_iter().collect()
    }
}
