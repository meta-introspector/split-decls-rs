use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T> PartialEq for IdxRange<T> {
    fn eq(&self, other: &Self) -> bool {
        self.range == other.range
    }
}
