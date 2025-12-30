// Generated macro for encode_not_indexed (function)
macro_rules! Depcrate_hpack_encoderencode_not_indexed {
() => {
// Module: crate::hpack::encoder
// Provides: {"encode_not_indexed"}
// Dependencies: {}
fn encode_not_indexed (name : usize , value : & [u8] , sensitive : bool , dst : & mut BytesMut) { if sensitive { encode_int (name , 4 , 0b10000 , dst) ; } else { encode_int (name , 4 , 0 , dst) ; } encode_str (value , dst) ; }
};
}
