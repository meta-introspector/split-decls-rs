// Generated macro for macro_425 (macro)
macro_rules! Depcrate_ffi_http_typesmacro_425 {
() => {
// Module: crate::ffi::http_types
// Provides: {"macro_425"}
// Dependencies: {}
ffi_fn ! { # [doc = " Free an HTTP request."] # [doc = ""] # [doc = " This should only be used if the request isn't consumed by"] # [doc = " `hyper_clientconn_send`."] fn hyper_request_free (req : * mut hyper_request) { drop (non_null ! (Box :: from_raw (req) ?= ())) ; } }
};
}
