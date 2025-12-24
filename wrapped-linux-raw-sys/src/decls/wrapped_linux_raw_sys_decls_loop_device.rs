use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "loop_device")]
#[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
#[path = "x32/loop_device.rs"]
pub mod loop_device;
