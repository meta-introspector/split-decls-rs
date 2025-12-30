// Generated macro for is_hex_digit_lc (function)
macro_rules! Depcrate_parseis_hex_digit_lc {
() => {
// Module: crate::parse
// Provides: {"is_hex_digit_lc"}
// Dependencies: {}
fn is_hex_digit_lc (b : u8) -> bool { matches ! (b , b'0' ..= b'9' | b'a' ..= b'f') }
};
}
