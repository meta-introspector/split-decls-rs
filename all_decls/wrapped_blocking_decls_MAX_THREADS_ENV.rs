use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Env variable that allows to override default value for max threads.
#[cfg(not(target_family = "wasm"))]
const MAX_THREADS_ENV: &str = "BLOCKING_MAX_THREADS";
