// Generated macro for encode_string (function)
macro_rules! Depcrate_upperencode_string {
() => {
// Module: crate::upper
// Provides: {"encode_string"}
// Dependencies: {}
# [doc = " Encode input byte slice into a [`String`] containing upper Base16 (hex)."] # [doc = ""] # [doc = " # Panics"] # [doc = " If `input` length is greater than `usize::MAX/2`."] # [cfg (feature = "alloc")] pub fn encode_string (input : & [u8]) -> String { let elen = encoded_len (input) ; let mut dst = vec ! [0u8 ; elen] ; let res = encode (input , & mut dst) . expect ("dst length is correct") ; debug_assert_eq ! (elen , res . len ()) ; unsafe { String :: from_utf8_unchecked (dst) } }
};
}
