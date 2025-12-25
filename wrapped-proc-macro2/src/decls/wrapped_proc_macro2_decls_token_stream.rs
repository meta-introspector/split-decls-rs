use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Public implementation details for the `TokenStream` type, such as iterators.
pub mod token_stream {
    use crate::marker::{ProcMacroAutoTraits, MARKER};
    pub use crate::TokenStream;
    use crate::{imp, TokenTree};
    use core::fmt::{self, Debug};
    /// An iterator over `TokenStream`'s `TokenTree`s.
    ///
    /// The iteration is "shallow", e.g. the iterator doesn't recurse into
    /// delimited groups, and returns whole groups as token trees.
    #[derive(Clone)]
    pub struct IntoIter {
        inner: imp::TokenTreeIter,
        _marker: ProcMacroAutoTraits,
    }
    impl Iterator for IntoIter {
        type Item = TokenTree;
        fn next(&mut self) -> Option<TokenTree> {
            self.inner.next()
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            self.inner.size_hint()
        }
    }
    impl Debug for IntoIter {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("TokenStream ")?;
            f.debug_list().entries(self.clone()).finish()
        }
    }
    impl IntoIterator for TokenStream {
        type Item = TokenTree;
        type IntoIter = IntoIter;
        fn into_iter(self) -> IntoIter {
            IntoIter {
                inner: self.inner.into_iter(),
                _marker: MARKER,
            }
        }
    }
}
