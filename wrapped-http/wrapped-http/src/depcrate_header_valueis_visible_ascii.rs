// Generated macro for is_visible_ascii (function)
macro_rules! Depcrate_header_valueis_visible_ascii {
() => {
// Module: crate::header::value
// Provides: {"is_visible_ascii"}
// Dependencies: {}
const fn is_visible_ascii (b : u8) -> bool { b >= 32 && b < 127 || b == b'\t' }
};
}
