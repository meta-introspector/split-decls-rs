use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Converts a quantity of petabytes to bytes.
pub fn pb<V: Into<u64>>(size: V) -> u64 {
    size.into() * PB
}
