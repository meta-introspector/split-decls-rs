// Generated macro for rustls_slice_str_len (function)
macro_rules! Depcrate_rslicerustls_slice_str_len {
() => {
// Module: crate::rslice
// Provides: {"rustls_slice_str_len"}
// Dependencies: {}
# [doc = " Return the length of the outer slice."] # [doc = ""] # [doc = " If the input pointer is NULL, returns 0."] # [no_mangle] pub extern "C" fn rustls_slice_str_len (input : * const rustls_slice_str) -> size_t { unsafe { match input . as_ref () { Some (c) => c . inner . len () , None => 0 , } } }
};
}
