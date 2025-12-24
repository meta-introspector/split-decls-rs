use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<I> DoubleEndedIterator for Iterator<I>
where
    I: DoubleEndedFallibleIterator,
{
    #[inline]
    fn next_back(&mut self) -> Option<Result<I::Item, I::Error>> {
        match self.0.next_back() {
            Ok(Some(v)) => Some(Ok(v)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}
