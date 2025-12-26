use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Returns `core::cmp::Ordering::Equal`
impl Ord for Equal {
    #[inline]
    fn to_ordering() -> Ordering {
        Ordering::Equal
    }
}
