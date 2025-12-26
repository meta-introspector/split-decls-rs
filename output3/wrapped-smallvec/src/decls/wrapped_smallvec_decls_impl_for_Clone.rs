use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T: Clone, const N: usize> Clone for IntoIter<T, N> {
    #[inline]
    fn clone(&self) -> IntoIter<T, N> {
        SmallVec::from(self.as_slice()).into_iter()
    }
}
