use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(blake3_avx2_ffi)]
#[path = "ffi_avx2.rs"]
mod avx2;
