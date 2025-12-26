use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T: Primitive> From<MaybeUninit<T>> for AtomicMaybeUninit<T> {
    /// Creates a new atomic value from a potentially uninitialized value.
    #[inline]
    fn from(v: MaybeUninit<T>) -> Self {
        Self::new(v)
    }
}
