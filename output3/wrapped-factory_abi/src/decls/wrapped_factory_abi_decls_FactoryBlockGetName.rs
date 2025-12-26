use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub type FactoryBlockGetName = extern "C" fn(block_ptr: *mut c_void) -> *const c_char;
