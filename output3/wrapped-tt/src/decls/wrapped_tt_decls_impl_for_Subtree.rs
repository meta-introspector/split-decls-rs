use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<S> Subtree<S> {
    /// Count the number of tokens recursively
    pub fn count(&self) -> usize {
        self.usize_len()
    }
}
