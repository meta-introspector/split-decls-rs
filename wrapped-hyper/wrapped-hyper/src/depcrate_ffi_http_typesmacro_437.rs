// Generated macro for macro_437 (macro)
macro_rules! Depcrate_ffi_http_typesmacro_437 {
() => {
// Module: crate::ffi::http_types
// Provides: {"macro_437"}
// Dependencies: {}
ffi_fn ! { # [doc = " Get the length of the reason-phrase of this response."] # [doc = ""] # [doc = " Use `hyper_response_reason_phrase()` to get the buffer pointer."] fn hyper_response_reason_phrase_len (resp : * const hyper_response) -> size_t { non_null ! (&* resp ?= 0) . reason_phrase () . len () } }
};
}
