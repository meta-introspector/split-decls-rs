use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(any(windows, target_os = "cygwin"))]
mod windows_sys;
