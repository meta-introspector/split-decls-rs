// Generated macro for rustls_slice_str (struct)
macro_rules! Depcrate_rslicerustls_slice_str {
() => {
// Module: crate::rslice
// Provides: {"rustls_slice_str"}
// Dependencies: {}
# [doc = " A read-only view of a slice of multiple Rust `&str`'s (that is, multiple"] # [doc = " strings)."] # [doc = ""] # [doc = " Like `rustls_str`, this guarantees that each string contains"] # [doc = " UTF-8 and no NUL bytes. Strings are not NUL-terminated."] # [doc = ""] # [doc = " This is used to pass data from rustls-ffi to callback functions provided"] # [doc = " by the user of the API. Because Vec and slice are not `#[repr(C)]`, we"] # [doc = " can't provide a straightforward `data` and `len` structure. Instead, we"] # [doc = " provide access via a pointer to an opaque struct and accessor methods."] # [doc = " Internally, the pointee is a `&[&str]`."] # [doc = ""] # [doc = " The memory exposed is available as specified by the function"] # [doc = " using this in its signature. For instance, when this is a parameter to a"] # [doc = " callback, the lifetime will usually be the duration of the callback."] # [doc = " Functions that receive one of these must not call its methods beyond the"] # [doc = " allowed lifetime."] pub struct rustls_slice_str < 'a > { pub (crate) inner : & 'a [& 'a str] , }
};
}
