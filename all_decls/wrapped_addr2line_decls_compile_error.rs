use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "cargo-all")]
compile_error!("'--all-features' is not supported; use '--features all' instead");
