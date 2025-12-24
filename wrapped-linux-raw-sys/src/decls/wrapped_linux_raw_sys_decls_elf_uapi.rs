use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "elf_uapi")]
#[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
#[path = "x32/elf_uapi.rs"]
pub mod elf_uapi;
