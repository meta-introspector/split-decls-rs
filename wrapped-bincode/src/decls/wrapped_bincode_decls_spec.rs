use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(all(feature = "alloc", feature = "derive", doc))]
pub mod spec {
    #![doc = include_str!("../docs/spec.md")]
}
