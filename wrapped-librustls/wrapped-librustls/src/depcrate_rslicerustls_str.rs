// Generated macro for rustls_str (struct)
macro_rules! Depcrate_rslicerustls_str {
() => {
// Module: crate::rslice
// Provides: {"rustls_str"}
// Dependencies: {}
# [doc = " A read-only view on a Rust `&str`."] # [doc = ""] # [doc = " The contents are guaranteed to be valid UTF-8."] # [doc = ""] # [doc = " As an additional guarantee on top of Rust's normal UTF-8 guarantee,"] # [doc = " a `rustls_str` is guaranteed not to contain internal NUL bytes, so it is"] # [doc = " safe to interpolate into a C string or compare using strncmp. Keep in mind"] # [doc = " that it is not NUL-terminated."] # [doc = ""] # [doc = " The memory exposed is available as specified by the function"] # [doc = " using this in its signature. For instance, when this is a parameter to a"] # [doc = " callback, the lifetime will usually be the duration of the callback."] # [doc = " Functions that receive one of these must not dereference the data pointer"] # [doc = " beyond the allowed lifetime."] # [repr (C)] pub struct rustls_str < 'a > { pub data : * const c_char , pub len : size_t , phantom : PhantomData < & 'a str > , }
};
}
