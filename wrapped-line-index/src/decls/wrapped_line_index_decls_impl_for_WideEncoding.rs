use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl WideEncoding {
    /// Returns the number of code units it takes to encode `text` in this encoding.
    pub fn measure(&self, text: &str) -> usize {
        match self {
            WideEncoding::Utf16 => text.encode_utf16().count(),
            WideEncoding::Utf32 => text.chars().count(),
        }
    }
}
