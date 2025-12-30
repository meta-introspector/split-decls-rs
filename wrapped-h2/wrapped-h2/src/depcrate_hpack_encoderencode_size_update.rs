// Generated macro for encode_size_update (function)
macro_rules! Depcrate_hpack_encoderencode_size_update {
() => {
// Module: crate::hpack::encoder
// Provides: {"encode_size_update"}
// Dependencies: {}
fn encode_size_update (val : usize , dst : & mut BytesMut) { encode_int (val , 5 , 0b0010_0000 , dst) }
};
}
