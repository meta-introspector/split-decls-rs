// Generated macro for impl_82 (impl)
macro_rules! Depcrate_errorimpl_82 {
() => {
// Module: crate::error
// Provides: {"impl_82"}
// Dependencies: {}
# [cfg (any (feature = "std" , not (anyhow_no_core_error)))] impl < E > From < E > for Error where E : StdError + Send + Sync + 'static , { # [cold] fn from (error : E) -> Self { let backtrace = backtrace_if_absent ! (& error) ; Error :: construct_from_std (error , backtrace) } }
};
}
