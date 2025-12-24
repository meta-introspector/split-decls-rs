use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(all(target_arch = "wasm32", feature = "wasm-bindgen"))]
mod wasm;
