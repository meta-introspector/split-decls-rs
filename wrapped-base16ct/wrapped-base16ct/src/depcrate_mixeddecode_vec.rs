// Generated macro for decode_vec (function)
macro_rules! Depcrate_mixeddecode_vec {
() => {
// Module: crate::mixed
// Provides: {"decode_vec"}
// Dependencies: {}
# [doc = " Decode a mixed Base16 (hex) string into a byte vector."] # [cfg (feature = "alloc")] pub fn decode_vec (input : impl AsRef < [u8] >) -> Result < Vec < u8 > , Error > { let mut output = vec ! [0u8 ; decoded_len (input . as_ref ()) ?] ; decode (input , & mut output) ? ; Ok (output) }
};
}
