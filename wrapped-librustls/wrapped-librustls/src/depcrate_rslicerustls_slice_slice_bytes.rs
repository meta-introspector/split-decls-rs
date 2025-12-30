// Generated macro for rustls_slice_slice_bytes (struct)
macro_rules! Depcrate_rslicerustls_slice_slice_bytes {
() => {
// Module: crate::rslice
// Provides: {"rustls_slice_slice_bytes"}
// Dependencies: {}
# [doc = " A read-only view of a slice of Rust byte slices."] # [doc = ""] # [doc = " This is used to pass data from rustls-ffi to callback functions provided"] # [doc = " by the user of the API. Because Vec and slice are not `#[repr(C)]`, we"] # [doc = " provide access via a pointer to an opaque struct and an accessor method"] # [doc = " that acts on that struct to get entries of type `rustls_slice_bytes`."] # [doc = " Internally, the pointee is a `&[&[u8]]`."] # [doc = ""] # [doc = " The memory exposed is available as specified by the function"] # [doc = " using this in its signature. For instance, when this is a parameter to a"] # [doc = " callback, the lifetime will usually be the duration of the callback."] # [doc = " Functions that receive one of these must not call its methods beyond the"] # [doc = " allowed lifetime."] pub struct rustls_slice_slice_bytes < 'a > { pub (crate) inner : & 'a [& 'a [u8]] , }
};
}
