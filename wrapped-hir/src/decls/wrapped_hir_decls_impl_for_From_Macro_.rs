use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl From<Macro> for ItemInNs {
    fn from(it: Macro) -> Self {
        Self::Macros(it)
    }
}
