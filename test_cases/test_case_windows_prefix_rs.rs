// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/path/windows_prefix.rs
// Error: expected square brackets
// Problematic line: line 7

use crate::ffi::OsStr;
use crate::path::Prefix;

struct PrefixParser<'a, const LEN: usize> {
    path: &'a OsStr,
    prefix: [u8; LEN],
}
