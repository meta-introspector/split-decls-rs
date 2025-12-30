// Generated macro for is_ascii_alphanumeric (function)
macro_rules! Depcrate_scannersis_ascii_alphanumeric {
() => {
// Module: crate::scanners
// Provides: {"is_ascii_alphanumeric"}
// Dependencies: {}
pub (crate) fn is_ascii_alphanumeric (c : u8) -> bool { matches ! (c , b'0' ..= b'9' | b'a' ..= b'z' | b'A' ..= b'Z') }
};
}
