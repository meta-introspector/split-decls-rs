use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub fn memory_usage() -> MemoryUsage {
    MemoryUsage::now()
}
