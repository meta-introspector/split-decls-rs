// Generated macro for opt_bytes_to_ptr (function)
macro_rules! Depcrate_ffi_utilopt_bytes_to_ptr {
() => {
// Module: crate::ffi_util
// Provides: {"opt_bytes_to_ptr"}
// Dependencies: {}
# [doc = " Returns a raw pointer to borrowed bytes, or null if None."] # [doc = ""] # [doc = " # Safety"] # [doc = " - The input must outlive the returned pointer."] # [doc = " - Common types: `&str`, `&[u8]`, `&String`, `&Vec<u8>`"] pub fn opt_bytes_to_ptr < T : AsRef < [u8] > + ? Sized > (opt : Option < & T >) -> * const c_char { match opt { Some (v) => v . as_ref () . as_ptr () as * const c_char , None => ptr :: null () , } }
};
}
