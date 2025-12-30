// Generated macro for macro_361 (macro)
macro_rules! Depcrate_ffi_bodymacro_361 {
() => {
// Module: crate::ffi::body
// Provides: {"macro_361"}
// Dependencies: {}
ffi_fn ! { # [doc = " Free a body."] # [doc = ""] # [doc = " This should only be used if the request isn't consumed by"] # [doc = " `hyper_body_foreach` or `hyper_request_set_body`."] fn hyper_body_free (body : * mut hyper_body) { drop (non_null ! (Box :: from_raw (body) ?= ())) ; } }
};
}
