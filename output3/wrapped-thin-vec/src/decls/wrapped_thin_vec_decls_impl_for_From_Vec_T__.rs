use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T> From<Vec<T>> for ThinVec<T> {
    /// Convert a `std::Vec` into a `ThinVec`.
    ///
    /// **NOTE:** this must reallocate to change the layout!
    ///
    /// # Examples
    ///
    /// ```
    /// use thin_vec::{ThinVec, thin_vec};
    ///
    /// let b: Vec<i32> = vec![1, 2, 3];
    /// assert_eq!(ThinVec::from(b), thin_vec![1, 2, 3]);
    /// ```
    fn from(s: Vec<T>) -> Self {
        s.into_iter().collect()
    }
}
