// Generated macro for macro_434 (macro)
macro_rules! Depcrate_ffi_http_typesmacro_434 {
() => {
// Module: crate::ffi::http_types
// Provides: {"macro_434"}
// Dependencies: {}
ffi_fn ! { # [doc = " Free an HTTP response."] # [doc = ""] # [doc = " This should be used for any response once it is no longer needed."] fn hyper_response_free (resp : * mut hyper_response) { drop (non_null ! (Box :: from_raw (resp) ?= ())) ; } }
};
}
