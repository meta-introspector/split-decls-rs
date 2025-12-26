use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T, const N: usize> Ord for SmallVec<T, N>
where
    T: Ord,
{
    #[inline]
    fn cmp(&self, other: &SmallVec<T, N>) -> core::cmp::Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}
