use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl_Integer_size!(usize as u64 #[cfg(target_pointer_width = "64")]);
