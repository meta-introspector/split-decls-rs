use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl PartialOrd for Utf8Path {
    #[inline]
    fn partial_cmp(&self, other: &Utf8Path) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
