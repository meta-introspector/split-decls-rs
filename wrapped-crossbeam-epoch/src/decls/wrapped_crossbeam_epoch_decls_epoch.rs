use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(all(feature = "alloc", target_has_atomic = "ptr"))]
mod epoch;
