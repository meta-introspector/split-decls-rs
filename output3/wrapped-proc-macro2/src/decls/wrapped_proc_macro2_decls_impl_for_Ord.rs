use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Ord for Ident {
    fn cmp(&self, other: &Ident) -> Ordering {
        self.to_string().cmp(&other.to_string())
    }
}
