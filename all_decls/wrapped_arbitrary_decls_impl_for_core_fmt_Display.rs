use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl core::fmt::Display for MaxRecursionReached {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Maximum recursion depth has been reached")
    }
}
