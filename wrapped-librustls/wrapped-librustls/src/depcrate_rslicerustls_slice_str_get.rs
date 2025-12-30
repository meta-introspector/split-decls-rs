// Generated macro for rustls_slice_str_get (function)
macro_rules! Depcrate_rslicerustls_slice_str_get {
() => {
// Module: crate::rslice
// Provides: {"rustls_slice_str_get"}
// Dependencies: {}
# [doc = " Retrieve the nth element from the input slice of `&str`s."] # [doc = ""] # [doc = " If the input pointer is NULL, or n is greater than the length of the"] # [doc = " rustls_slice_str, returns rustls_str{NULL, 0}."] # [no_mangle] pub extern "C" fn rustls_slice_str_get (input : * const rustls_slice_str , n : size_t) -> rustls_str { let input : & rustls_slice_str = unsafe { match input . as_ref () { Some (c) => c , None => { return rustls_str { data : null () , len : 0 , phantom : PhantomData , } ; } } } ; input . inner . get (n) . and_then (| & s | s . try_into () . ok ()) . unwrap_or (rustls_str { data : null () , len : 0 , phantom : PhantomData , }) }
};
}
