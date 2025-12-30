// Generated macro for encode_block (function)
macro_rules! Depcrate_base64encode_block {
() => {
// Module: crate::base64
// Provides: {"encode_block"}
// Dependencies: {}
# [doc = " Encodes a slice of bytes to a base64 string."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the input length or computed output length overflow a signed C integer."] # [corresponds (EVP_EncodeBlock)] pub fn encode_block (src : & [u8]) -> String { assert ! (src . len () <= c_int :: MAX as usize) ; let src_len = src . len () as LenType ; let len = encoded_len (src_len) . unwrap () ; let mut out = Vec :: with_capacity (len as usize) ; unsafe { let out_len = ffi :: EVP_EncodeBlock (out . as_mut_ptr () , src . as_ptr () , src_len) ; out . set_len (out_len as usize) ; String :: from_utf8_unchecked (out) } }
};
}
