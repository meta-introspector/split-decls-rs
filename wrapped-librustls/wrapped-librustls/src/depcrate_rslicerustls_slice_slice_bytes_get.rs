// Generated macro for rustls_slice_slice_bytes_get (function)
macro_rules! Depcrate_rslicerustls_slice_slice_bytes_get {
() => {
// Module: crate::rslice
// Provides: {"rustls_slice_slice_bytes_get"}
// Dependencies: {}
# [doc = " Retrieve the nth element from the input slice of slices."] # [doc = ""] # [doc = " If the input pointer is NULL, or n is greater than the length"] # [doc = " of the `rustls_slice_slice_bytes`, returns rustls_slice_bytes{NULL, 0}."] # [no_mangle] pub extern "C" fn rustls_slice_slice_bytes_get (input : * const rustls_slice_slice_bytes , n : size_t ,) -> rustls_slice_bytes { let input = { match unsafe { input . as_ref () } { Some (c) => c , None => { return rustls_slice_bytes { data : null () , len : 0 , phantom : PhantomData , } ; } } } ; match input . inner . get (n) { Some (rsb) => (* rsb) . into () , None => rustls_slice_bytes { data : null () , len : 0 , phantom : PhantomData , } , } }
};
}
