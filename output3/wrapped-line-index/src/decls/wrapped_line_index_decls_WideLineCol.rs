use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// `(line, column)` information in wide encodings.
///
/// See [`WideEncoding`] for the kinds of wide encodings available.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WideLineCol {
    /// Zero-based.
    pub line: u32,
    /// Zero-based.
    pub col: u32,
}
