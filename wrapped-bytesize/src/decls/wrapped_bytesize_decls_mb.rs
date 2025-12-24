use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Converts a quantity of megabytes to bytes.
pub fn mb<V: Into<u64>>(size: V) -> u64 {
    size.into() * MB
}
