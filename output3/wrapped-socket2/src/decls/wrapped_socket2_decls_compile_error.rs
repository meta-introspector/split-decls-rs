use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(any(windows, unix)))]
compile_error!("Socket2 doesn't support the compile target");
