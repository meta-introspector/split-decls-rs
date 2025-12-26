use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T, const N: usize> Default for SmallVec<T, N> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
