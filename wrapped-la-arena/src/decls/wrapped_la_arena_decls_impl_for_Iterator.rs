use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T> Iterator for IntoIter<T> {
    type Item = (Idx<T>, T);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(idx, value)| (Idx::from_raw(RawIdx(idx as u32)), value))
    }
}
