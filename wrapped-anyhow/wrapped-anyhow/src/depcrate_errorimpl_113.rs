// Generated macro for impl_113 (impl)
macro_rules! Depcrate_errorimpl_113 {
() => {
// Module: crate::error
// Provides: {"impl_113"}
// Dependencies: {}
# [cfg (any (feature = "std" , not (anyhow_no_core_error)))] impl From < Error > for Box < dyn StdError + Send + Sync + 'static > { # [cold] fn from (error : Error) -> Self { error . into_boxed_dyn_error () } }
};
}
