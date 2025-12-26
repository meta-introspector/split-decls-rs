use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(windows))]
pub fn symlink_supported() -> bool {
    true
}
