// Generated macro for encode_not_indexed2 (function)
macro_rules! Depcrate_hpack_encoderencode_not_indexed2 {
() => {
// Module: crate::hpack::encoder
// Provides: {"encode_not_indexed2"}
// Dependencies: {}
fn encode_not_indexed2 (name : & [u8] , value : & [u8] , sensitive : bool , dst : & mut BytesMut) { if sensitive { dst . put_u8 (0b10000) ; } else { dst . put_u8 (0) ; } encode_str (name , dst) ; encode_str (value , dst) ; }
};
}
