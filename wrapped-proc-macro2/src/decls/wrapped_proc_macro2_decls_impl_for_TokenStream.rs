use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl TokenStream {
    fn _new(inner: imp::TokenStream) -> Self {
        TokenStream {
            inner,
            _marker: MARKER,
        }
    }
    fn _new_fallback(inner: fallback::TokenStream) -> Self {
        TokenStream {
            inner: imp::TokenStream::from(inner),
            _marker: MARKER,
        }
    }
    /// Returns an empty `TokenStream` containing no token trees.
    pub fn new() -> Self {
        TokenStream::_new(imp::TokenStream::new())
    }
    /// Checks if this `TokenStream` is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}
