use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl fmt::Display for ReferenceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.str().fmt(f)
    }
}
