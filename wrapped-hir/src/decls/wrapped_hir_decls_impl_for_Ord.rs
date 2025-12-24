use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Ord for Local {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.binding_id.cmp(&other.binding_id)
    }
}
