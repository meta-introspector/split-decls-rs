// Generated macro for macro_360 (macro)
macro_rules! Depcrate_ffi_bodymacro_360 {
() => {
// Module: crate::ffi::body
// Provides: {"macro_360"}
// Dependencies: {}
ffi_fn ! { # [doc = " Creates a new \"empty\" body."] # [doc = ""] # [doc = " If not configured, this body acts as an empty payload."] # [doc = ""] # [doc = " To avoid a memory leak, the body must eventually be consumed by"] # [doc = " `hyper_body_free`, `hyper_body_foreach`, or `hyper_request_set_body`."] fn hyper_body_new () -> * mut hyper_body { Box :: into_raw (Box :: new (hyper_body (IncomingBody :: ffi ()))) } ?= ptr :: null_mut () }
};
}
