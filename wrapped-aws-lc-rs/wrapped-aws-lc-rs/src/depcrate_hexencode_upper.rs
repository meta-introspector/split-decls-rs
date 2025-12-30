// Generated macro for encode_upper (function)
macro_rules! Depcrate_hexencode_upper {
() => {
// Module: crate::hex
// Provides: {"encode_upper"}
// Dependencies: {}
# [doc = " Converts bytes to an upper-case hex string"] pub fn encode_upper < T : AsRef < [u8] > > (bytes : T) -> String { encode (bytes) . to_ascii_uppercase () }
};
}
