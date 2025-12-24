use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(blake3_sse41_ffi)]
#[path = "ffi_sse41.rs"]
mod sse41;
