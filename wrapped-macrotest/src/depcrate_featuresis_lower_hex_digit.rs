// Generated macro for is_lower_hex_digit (function)
macro_rules! Depcrate_featuresis_lower_hex_digit {
() => {
// Module: crate::features
// Provides: {"is_lower_hex_digit"}
// Dependencies: {}
fn is_lower_hex_digit (byte : u8) -> bool { matches ! (byte , b'0' ..= b'9' | b'a' ..= b'f') }
};
}
