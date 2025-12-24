use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(blake3_sse2_ffi)]
#[path = "ffi_sse2.rs"]
mod sse2;
