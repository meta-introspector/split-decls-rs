// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/path/unsupported_backslash.rs
// Error: expected square brackets
// Problematic line: line 7

use crate::path::{Path, PathBuf, Prefix};
use crate::sys::unsupported;

#[inline]
pub fn is_sep_byte(b: u8) -> bool {
    b == b'\\'
}
