use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "landlock")]
#[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
#[path = "x32/landlock.rs"]
pub mod landlock;
