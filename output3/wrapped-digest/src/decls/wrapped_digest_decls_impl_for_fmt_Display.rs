use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl fmt::Display for InvalidBufferSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid buffer length")
    }
}
