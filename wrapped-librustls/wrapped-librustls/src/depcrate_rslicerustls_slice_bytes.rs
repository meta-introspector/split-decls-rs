// Generated macro for rustls_slice_bytes (struct)
macro_rules! Depcrate_rslicerustls_slice_bytes {
() => {
// Module: crate::rslice
// Provides: {"rustls_slice_bytes"}
// Dependencies: {}
# [doc = " A read-only view on a Rust byte slice."] # [doc = ""] # [doc = " This is used to pass data from rustls-ffi to callback functions provided"] # [doc = " by the user of the API."] # [doc = " `len` indicates the number of bytes than can be safely read."] # [doc = ""] # [doc = " The memory exposed is available as specified by the function"] # [doc = " using this in its signature. For instance, when this is a parameter to a"] # [doc = " callback, the lifetime will usually be the duration of the callback."] # [doc = " Functions that receive one of these must not dereference the data pointer"] # [doc = " beyond the allowed lifetime."] # [repr (C)] pub struct rustls_slice_bytes < 'a > { pub data : * const u8 , pub len : size_t , phantom : PhantomData < & 'a [u8] > , }
};
}
