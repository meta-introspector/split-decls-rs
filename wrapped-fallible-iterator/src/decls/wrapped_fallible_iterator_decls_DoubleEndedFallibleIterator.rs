use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A fallible iterator able to yield elements from both ends.
pub trait DoubleEndedFallibleIterator: FallibleIterator {
    /// Advances the end of the iterator, returning the last value.
    fn next_back(&mut self) -> Result<Option<Self::Item>, Self::Error>;
    /// Applies a function over the elements of the iterator in reverse order, producing a single final value.
    #[inline]
    fn rfold<B, F>(mut self, init: B, f: F) -> Result<B, Self::Error>
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> Result<B, Self::Error>,
    {
        self.try_rfold(init, f)
    }
    /// Applies a function over the elements of the iterator in reverse, producing a single final value.
    ///
    /// This is used as the "base" of many methods on `DoubleEndedFallibleIterator`.
    #[inline]
    fn try_rfold<B, E, F>(&mut self, mut init: B, mut f: F) -> Result<B, E>
    where
        Self: Sized,
        E: From<Self::Error>,
        F: FnMut(B, Self::Item) -> Result<B, E>,
    {
        while let Some(v) = self.next_back()? {
            init = f(init, v)?;
        }
        Ok(init)
    }
}
