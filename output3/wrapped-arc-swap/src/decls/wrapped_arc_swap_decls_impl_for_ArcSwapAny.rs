use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T, S: Strategy<Option<Arc<T>>>> ArcSwapAny<Option<Arc<T>>, S> {
    /// A convenience constructor directly from a pointed-to value.
    ///
    /// This just allocates the `Arc` under the hood.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use arc_swap::ArcSwapOption;
    ///
    /// let empty: ArcSwapOption<usize> = ArcSwapOption::from_pointee(None);
    /// assert!(empty.load().is_none());
    /// let non_empty: ArcSwapOption<usize> = ArcSwapOption::from_pointee(42);
    /// assert_eq!(42, **non_empty.load().as_ref().unwrap());
    /// ```
    pub fn from_pointee<V: Into<Option<T>>>(val: V) -> Self
    where
        S: Default,
    {
        Self::new(val.into().map(Arc::new))
    }
    /// A convenience constructor for an empty value.
    ///
    /// This is equivalent to `ArcSwapOption::new(None)`.
    pub fn empty() -> Self
    where
        S: Default,
    {
        Self::new(None)
    }
}
