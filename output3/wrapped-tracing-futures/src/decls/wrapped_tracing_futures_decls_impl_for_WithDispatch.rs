use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "std")]
impl<T> WithDispatch<T> {
    /// Wrap a future, stream, sink or executor with the same subscriber as this WithDispatch.
    pub fn with_dispatch<U>(&self, inner: U) -> WithDispatch<U> {
        WithDispatch {
            dispatch: self.dispatch.clone(),
            inner,
        }
    }
    /// Borrows the `Dispatch` that this type is instrumented by.
    pub fn dispatch(&self) -> &Dispatch {
        &self.dispatch
    }
    /// Get a pinned reference to the wrapped type.
    #[cfg(feature = "std-future")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std-future")))]
    pub fn inner_pin_ref(self: Pin<&Self>) -> Pin<&T> {
        self.project_ref().inner
    }
    /// Get a pinned mutable reference to the wrapped type.
    #[cfg(feature = "std-future")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std-future")))]
    pub fn inner_pin_mut(self: Pin<&mut Self>) -> Pin<&mut T> {
        self.project().inner
    }
    /// Borrows the wrapped type.
    pub fn inner(&self) -> &T {
        &self.inner
    }
    /// Mutably borrows the wrapped type.
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }
    /// Consumes the `WithDispatch`, returning the wrapped type.
    pub fn into_inner(self) -> T {
        self.inner
    }
}
