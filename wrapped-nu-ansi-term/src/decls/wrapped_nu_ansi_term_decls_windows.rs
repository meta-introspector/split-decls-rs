use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(all(windows, feature = "std"))]
mod windows;
