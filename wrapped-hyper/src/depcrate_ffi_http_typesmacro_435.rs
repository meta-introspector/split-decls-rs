// Generated macro for macro_435 (macro)
macro_rules! Depcrate_ffi_http_typesmacro_435 {
() => {
// Module: crate::ffi::http_types
// Provides: {"macro_435"}
// Dependencies: {}
ffi_fn ! { # [doc = " Get the HTTP-Status code of this response."] # [doc = ""] # [doc = " It will always be within the range of 100-599."] fn hyper_response_status (resp : * const hyper_response) -> u16 { non_null ! (&* resp ?= 0) . 0 . status () . as_u16 () } }
};
}
