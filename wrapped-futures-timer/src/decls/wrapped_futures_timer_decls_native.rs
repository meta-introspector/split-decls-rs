use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(all(target_arch = "wasm32", feature = "wasm-bindgen")))]
mod native;
