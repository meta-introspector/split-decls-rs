use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg_attr(unix, path = "unix.rs")]
#[cfg_attr(windows, path = "windows.rs")]
#[cfg_attr(not(any(unix, windows)), path = "stub.rs")]
mod os;
