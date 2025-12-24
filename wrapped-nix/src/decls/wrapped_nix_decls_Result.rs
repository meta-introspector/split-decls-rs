use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Nix Result Type
pub type Result<T> = result::Result<T, Errno>;
