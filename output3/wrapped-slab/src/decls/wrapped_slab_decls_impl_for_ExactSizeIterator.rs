use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T> ExactSizeIterator for Drain<'_, T> {
    fn len(&self) -> usize {
        self.len
    }
}
