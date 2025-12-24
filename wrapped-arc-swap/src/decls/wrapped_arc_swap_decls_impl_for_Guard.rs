use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T: RefCnt, S: Strategy<T>> Guard<T, S> {
    /// Converts it into the held value.
    ///
    /// This, on occasion, may be a tiny bit faster than cloning the Arc or whatever is being held
    /// inside.
    #[allow(clippy::wrong_self_convention)]
    #[inline]
    pub fn into_inner(lease: Self) -> T {
        lease.inner.into_inner()
    }
    /// Create a guard for a given value `inner`.
    ///
    /// This can be useful on occasion to pass a specific object to code that expects or
    /// wants to store a Guard.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use arc_swap::{ArcSwap, DefaultStrategy, Guard};
    /// # use std::sync::Arc;
    /// # let p = ArcSwap::from_pointee(42);
    /// // Create two guards pointing to the same object
    /// let g1 = p.load();
    /// let g2 = Guard::<_, DefaultStrategy>::from_inner(Arc::clone(&*g1));
    /// # drop(g2);
    /// ```
    pub fn from_inner(inner: T) -> Self {
        Guard {
            inner: S::Protected::from_inner(inner),
        }
    }
}
