use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
compile_error!(
    "This wasm target is unsupported by mio. If using Tokio, disable the net feature."
);
