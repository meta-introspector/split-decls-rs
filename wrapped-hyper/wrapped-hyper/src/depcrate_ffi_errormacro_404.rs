// Generated macro for macro_404 (macro)
macro_rules! Depcrate_ffi_errormacro_404 {
() => {
// Module: crate::ffi::error
// Provides: {"macro_404"}
// Dependencies: {}
ffi_fn ! { # [doc = " Frees a `hyper_error`."] # [doc = ""] # [doc = " This should be used for any error once it is no longer needed."] fn hyper_error_free (err : * mut hyper_error) { drop (non_null ! (Box :: from_raw (err) ?= ())) ; } }
};
}
