use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(feature = "deadlock_detection"))]
mod deadlock;
