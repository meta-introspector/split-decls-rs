use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<I> IntoFallibleIterator for I
where
    I: FallibleIterator,
{
    type Item = I::Item;
    type Error = I::Error;
    type IntoFallibleIter = I;
    #[inline]
    fn into_fallible_iter(self) -> I {
        self
    }
}
