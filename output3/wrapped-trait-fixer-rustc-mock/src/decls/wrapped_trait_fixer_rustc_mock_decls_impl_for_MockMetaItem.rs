use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl MockMetaItem {
    pub fn has_name(self, _symbol: Symbol) -> bool {
        true
    }
}
