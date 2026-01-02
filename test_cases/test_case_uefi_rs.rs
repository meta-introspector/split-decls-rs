// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/path/uefi.rs
// Error: expected square brackets
// Problematic line: line 10

const FORWARD_SLASH: u8 = b'/';
const COLON: u8 = b':';

#[inline]
pub fn is_sep_byte(b: u8) -> bool {
    b == b'\\'
}
