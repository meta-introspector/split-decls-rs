use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_str().fmt(f)
    }
}
