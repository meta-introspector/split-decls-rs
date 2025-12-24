use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "ptrace")]
#[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
#[path = "x32/ptrace.rs"]
pub mod ptrace;
