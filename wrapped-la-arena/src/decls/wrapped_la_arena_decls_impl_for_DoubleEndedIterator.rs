use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T> DoubleEndedIterator for IdxRange<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.range.next_back().map(|raw| Idx::from_raw(raw.into()))
    }
}
