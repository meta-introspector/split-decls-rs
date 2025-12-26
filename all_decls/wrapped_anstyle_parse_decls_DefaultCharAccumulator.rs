use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(feature = "utf8"))]
pub type DefaultCharAccumulator = AsciiParser;
