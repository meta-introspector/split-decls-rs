use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// `(line, column)` information in the native, UTF-8 encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LineCol {
    /// Zero-based.
    pub line: u32,
    /// Zero-based UTF-8 offset.
    pub col: u32,
}
