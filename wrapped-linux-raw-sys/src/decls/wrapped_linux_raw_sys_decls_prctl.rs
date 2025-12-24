use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "prctl")]
#[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
#[path = "x32/prctl.rs"]
pub mod prctl;
