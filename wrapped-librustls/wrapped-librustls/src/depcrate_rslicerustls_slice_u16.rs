// Generated macro for rustls_slice_u16 (struct)
macro_rules! Depcrate_rslicerustls_slice_u16 {
() => {
// Module: crate::rslice
// Provides: {"rustls_slice_u16"}
// Dependencies: {}
# [doc = " A read-only view on a Rust slice of 16-bit integers in platform endianness."] # [doc = ""] # [doc = " This is used to pass data from rustls-ffi to callback functions provided"] # [doc = " by the user of the API."] # [doc = " `len` indicates the number of bytes than can be safely read."] # [doc = ""] # [doc = " The memory exposed is available as specified by the function"] # [doc = " using this in its signature. For instance, when this is a parameter to a"] # [doc = " callback, the lifetime will usually be the duration of the callback."] # [doc = " Functions that receive one of these must not dereference the data pointer"] # [doc = " beyond the allowed lifetime."] # [repr (C)] pub struct rustls_slice_u16 < 'a > { pub data : * const u16 , pub len : size_t , phantom : PhantomData < & 'a [u16] > , }
};
}
