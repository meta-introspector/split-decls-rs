use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(feature = "terminfo"))]
mod ansi;
