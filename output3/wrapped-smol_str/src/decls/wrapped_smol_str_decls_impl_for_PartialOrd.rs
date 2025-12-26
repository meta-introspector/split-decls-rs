use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl PartialOrd for SmolStr {
    fn partial_cmp(&self, other: &SmolStr) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
