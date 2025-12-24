use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl fmt::LowerHex for Hash128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.inner, f)
    }
}
