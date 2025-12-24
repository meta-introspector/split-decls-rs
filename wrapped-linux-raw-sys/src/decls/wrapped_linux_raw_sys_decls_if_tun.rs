use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "if_tun")]
#[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
#[path = "x32/if_tun.rs"]
pub mod if_tun;
