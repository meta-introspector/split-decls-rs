use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T> iter::DoubleEndedIterator for Unwrap<T>
where
    T: DoubleEndedFallibleIterator,
    T::Error: core::fmt::Debug,
{
    #[inline]
    fn next_back(&mut self) -> Option<T::Item> {
        self.0.next_back().unwrap()
    }
}
