use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg_attr(target_os = "none", cfg(target_has_atomic = "ptr"))]
#[cfg(feature = "std")]
pub mod mpsc;
