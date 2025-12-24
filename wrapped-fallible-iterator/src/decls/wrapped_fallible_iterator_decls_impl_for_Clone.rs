use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<I> Clone for Flatten<I>
where
    I: FallibleIterator + Clone,
    I::Item: IntoFallibleIterator,
    <I::Item as IntoFallibleIterator>::IntoFallibleIter: Clone,
{
    #[inline]
    fn clone(&self) -> Flatten<I> {
        Flatten {
            it: self.it.clone(),
            cur: self.cur.clone(),
        }
    }
}
