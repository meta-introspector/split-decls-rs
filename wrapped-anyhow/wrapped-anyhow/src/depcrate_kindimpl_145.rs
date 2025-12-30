// Generated macro for impl_145 (impl)
macro_rules! Depcrate_kindimpl_145 {
() => {
// Module: crate::kind
// Provides: {"impl_145"}
// Dependencies: {}
# [cfg (any (feature = "std" , not (anyhow_no_core_error)))] impl Boxed { # [cold] pub fn new (self , error : Box < dyn StdError + Send + Sync >) -> Error { let backtrace = backtrace_if_absent ! (&* error) ; Error :: construct_from_boxed (error , backtrace) } }
};
}
