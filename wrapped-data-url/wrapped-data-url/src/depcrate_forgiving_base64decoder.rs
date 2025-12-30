// Generated macro for Decoder (struct)
macro_rules! Depcrate_forgiving_base64Decoder {
() => {
// Module: crate::forgiving_base64
// Provides: {"Decoder"}
// Dependencies: {}
# [doc = " <https://infra.spec.whatwg.org/#forgiving-base64-decode>"] pub struct Decoder < F , E > where F : FnMut (& [u8]) -> Result < () , E > , { write_bytes : F , bit_buffer : u32 , buffer_bit_length : u8 , padding_symbols : u8 , }
};
}
