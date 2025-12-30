// Generated macro for macro_439 (macro)
macro_rules! Depcrate_ffi_http_typesmacro_439 {
() => {
// Module: crate::ffi::http_types
// Provides: {"macro_439"}
// Dependencies: {}
ffi_fn ! { # [doc = " Gets a reference to the HTTP headers of this response."] # [doc = ""] # [doc = " This is not an owned reference, so it should not be accessed after the"] # [doc = " `hyper_response` has been freed."] fn hyper_response_headers (resp : * mut hyper_response) -> * mut hyper_headers { hyper_headers :: get_or_default (unsafe { & mut * resp } . 0 . extensions_mut ()) } ?= std :: ptr :: null_mut () }
};
}
