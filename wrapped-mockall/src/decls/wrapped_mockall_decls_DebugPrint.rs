use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[doc(hidden)]
pub struct DebugPrint<'a, T: Debug>(pub &'a T);
