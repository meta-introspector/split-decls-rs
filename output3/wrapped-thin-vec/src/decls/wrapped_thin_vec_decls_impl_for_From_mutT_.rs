use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T: Clone> From<&mut [T]> for ThinVec<T> {
    /// Allocate a `ThinVec<T>` and fill it by cloning `s`'s items.
    ///
    /// # Examples
    ///
    /// ```
    /// use thin_vec::{ThinVec, thin_vec};
    ///
    /// assert_eq!(ThinVec::from(&mut [1, 2, 3][..]), thin_vec![1, 2, 3]);
    /// ```
    fn from(s: &mut [T]) -> ThinVec<T> {
        s.iter().cloned().collect()
    }
}
