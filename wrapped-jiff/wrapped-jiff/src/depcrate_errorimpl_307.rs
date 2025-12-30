// Generated macro for impl_307 (impl)
macro_rules! Depcrate_errorimpl_307 {
() => {
// Module: crate::error
// Provides: {"impl_307"}
// Dependencies: {}
impl ErrorContext for Error { # [cfg_attr (feature = "perf-inline" , inline (always))] fn context (self , consequent : impl IntoError) -> Error { self . context_impl (consequent . into_error ()) } # [cfg_attr (feature = "perf-inline" , inline (always))] fn with_context < E : IntoError > (self , consequent : impl FnOnce () -> E ,) -> Error { self . context_impl (consequent () . into_error ()) } }
};
}
