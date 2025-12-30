// Generated macro for Decoder (struct)
macro_rules! Depcrate_hpack_decoderDecoder {
() => {
// Module: crate::hpack::decoder
// Provides: {"Decoder"}
// Dependencies: {}
# [doc = " Decodes headers using HPACK"] # [derive (Debug)] pub struct Decoder { max_size_update : Option < usize > , last_max_update : usize , table : Table , buffer : BytesMut , }
};
}
