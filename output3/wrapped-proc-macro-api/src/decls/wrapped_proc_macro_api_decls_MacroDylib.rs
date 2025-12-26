use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Represents a dynamically loaded library containing procedural macros.
pub struct MacroDylib {
    path: AbsPathBuf,
}
