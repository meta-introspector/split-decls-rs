// Generated macro for rustls_slice_slice_bytes_len (function)
macro_rules! Depcrate_rslicerustls_slice_slice_bytes_len {
() => {
// Module: crate::rslice
// Provides: {"rustls_slice_slice_bytes_len"}
// Dependencies: {}
# [doc = " Return the length of the outer slice. If the input pointer is NULL,"] # [doc = " returns 0."] # [no_mangle] pub extern "C" fn rustls_slice_slice_bytes_len (input : * const rustls_slice_slice_bytes) -> size_t { match unsafe { input . as_ref () } { Some (c) => c . inner . len () , None => 0 , } }
};
}
