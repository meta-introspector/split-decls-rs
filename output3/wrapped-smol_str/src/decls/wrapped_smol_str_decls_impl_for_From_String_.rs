use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl From<String> for SmolStr {
    #[inline(always)]
    fn from(text: String) -> Self {
        Self::new(text)
    }
}
