// Generated macro for hex_encode_upper (function)
macro_rules! Depcrate_encodehex_encode_upper {
() => {
// Module: crate::encode
// Provides: {"hex_encode_upper"}
// Dependencies: {}
pub fn hex_encode_upper < 'a > (src : & [u8] , dst : & 'a mut [u8]) -> Result < & 'a mut str , Error > { hex_encode_custom (src , dst , true) }
};
}
