use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T, E: Clone> FallibleIterator for RepeatErr<T, E> {
    type Item = T;
    type Error = E;
    #[inline]
    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        Err(self.1.clone())
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(0))
    }
}
