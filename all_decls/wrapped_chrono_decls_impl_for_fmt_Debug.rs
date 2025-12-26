use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl fmt::Debug for OutOfRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "out of range")
    }
}
