use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A kind of wide character encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum WideEncoding {
    /// UTF-16.
    Utf16,
    /// UTF-32.
    Utf32,
}
