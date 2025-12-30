// Generated macro for impl_308 (impl)
macro_rules! Depcrate_errorimpl_308 {
() => {
// Module: crate::error
// Provides: {"impl_308"}
// Dependencies: {}
impl < T > ErrorContext for Result < T , Error > { # [cfg_attr (feature = "perf-inline" , inline (always))] fn context (self , consequent : impl IntoError) -> Result < T , Error > { self . map_err (| err | err . context_impl (consequent . into_error ())) } # [cfg_attr (feature = "perf-inline" , inline (always))] fn with_context < E : IntoError > (self , consequent : impl FnOnce () -> E ,) -> Result < T , Error > { self . map_err (| err | err . context_impl (consequent () . into_error ())) } }
};
}
