use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(blake3_neon)]
#[path = "ffi_neon.rs"]
mod neon;
