use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Only allow parsing 7-bit ASCII
#[allow(clippy::exhaustive_structs)]
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct AsciiParser;
