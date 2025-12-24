use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "io_uring")]
#[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
#[path = "x32/io_uring.rs"]
pub mod io_uring;
