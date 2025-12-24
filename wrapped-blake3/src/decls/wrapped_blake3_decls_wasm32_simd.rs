use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(blake3_wasm32_simd)]
#[path = "wasm32_simd.rs"]
mod wasm32_simd;
