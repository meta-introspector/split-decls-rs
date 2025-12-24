use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "std")]
#[cfg(not(crossbeam_loom))]
pub mod thread;
