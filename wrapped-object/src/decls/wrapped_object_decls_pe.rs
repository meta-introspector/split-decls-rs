use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(any(feature = "coff", feature = "pe"))]
pub mod pe;
