// Generated macro for macro_405 (macro)
macro_rules! Depcrate_ffi_errormacro_405 {
() => {
// Module: crate::ffi::error
// Provides: {"macro_405"}
// Dependencies: {}
ffi_fn ! { # [doc = " Get an equivalent `hyper_code` from this error."] fn hyper_error_code (err : * const hyper_error) -> hyper_code { non_null ! (&* err ?= hyper_code :: HYPERE_INVALID_ARG) . code () } }
};
}
