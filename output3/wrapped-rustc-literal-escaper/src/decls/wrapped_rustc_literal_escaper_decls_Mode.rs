use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Enum of the different kinds of literal
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    /// `'a'`
    Char,
    /// `b'a'`
    Byte,
    /// `"hello"`
    Str,
    /// `r"hello"`
    RawStr,
    /// `b"hello"`
    ByteStr,
    /// `br"hello"`
    RawByteStr,
    /// `c"hello"`
    CStr,
    /// `cr"hello"`
    RawCStr,
}
