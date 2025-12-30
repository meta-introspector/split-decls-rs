// Generated macro for is_ascii_whitespace (function)
macro_rules! Depcrate_scannersis_ascii_whitespace {
() => {
// Module: crate::scanners
// Provides: {"is_ascii_whitespace"}
// Dependencies: {}
pub (crate) fn is_ascii_whitespace (c : u8) -> bool { (0x09 ..= 0x0d) . contains (& c) || c == b' ' }
};
}
