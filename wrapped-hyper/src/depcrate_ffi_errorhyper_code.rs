// Generated macro for hyper_code (enum)
macro_rules! Depcrate_ffi_errorhyper_code {
() => {
// Module: crate::ffi::error
// Provides: {"hyper_code"}
// Dependencies: {}
# [doc = " A return code for many of hyper's methods."] # [repr (C)] pub enum hyper_code { # [doc = " All is well."] HYPERE_OK , # [doc = " General error, details in the `hyper_error *`."] HYPERE_ERROR , # [doc = " A function argument was invalid."] HYPERE_INVALID_ARG , # [doc = " The IO transport returned an EOF when one wasn't expected."] # [doc = ""] # [doc = " This typically means an HTTP request or response was expected, but the"] # [doc = " connection closed cleanly without sending (all of) it."] HYPERE_UNEXPECTED_EOF , # [doc = " Aborted by a user supplied callback."] HYPERE_ABORTED_BY_CALLBACK , # [doc = " An optional hyper feature was not enabled."] # [cfg_attr (feature = "http2" , allow (unused))] HYPERE_FEATURE_NOT_ENABLED , # [doc = " The peer sent an HTTP message that could not be parsed."] HYPERE_INVALID_PEER_MESSAGE , }
};
}
