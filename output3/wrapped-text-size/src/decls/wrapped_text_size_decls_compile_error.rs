use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(target_pointer_width = "16")]
compile_error!("text-size assumes usize >= u32 and does not work on 16-bit targets");
