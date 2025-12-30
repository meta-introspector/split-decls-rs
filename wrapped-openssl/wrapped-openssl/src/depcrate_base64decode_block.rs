// Generated macro for decode_block (function)
macro_rules! Depcrate_base64decode_block {
() => {
// Module: crate::base64
// Provides: {"decode_block"}
// Dependencies: {}
# [doc = " Decodes a base64-encoded string to bytes."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the input length or computed output length overflow a signed C integer."] # [corresponds (EVP_DecodeBlock)] pub fn decode_block (src : & str) -> Result < Vec < u8 > , ErrorStack > { let src = src . trim () ; if src . is_empty () { return Ok (vec ! []) ; } assert ! (src . len () <= c_int :: MAX as usize) ; let src_len = src . len () as LenType ; let len = decoded_len (src_len) . unwrap () ; let mut out = Vec :: with_capacity (len as usize) ; unsafe { let out_len = cvt_n (ffi :: EVP_DecodeBlock (out . as_mut_ptr () , src . as_ptr () , src_len ,)) ? ; out . set_len (out_len as usize) ; } if src . ends_with ('=') { out . pop () ; if src . ends_with ("==") { out . pop () ; } } Ok (out) }
};
}
