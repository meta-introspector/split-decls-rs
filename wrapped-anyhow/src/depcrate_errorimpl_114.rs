// Generated macro for impl_114 (impl)
macro_rules! Depcrate_errorimpl_114 {
() => {
// Module: crate::error
// Provides: {"impl_114"}
// Dependencies: {}
# [cfg (any (feature = "std" , not (anyhow_no_core_error)))] impl From < Error > for Box < dyn StdError + Send + 'static > { # [cold] fn from (error : Error) -> Self { error . into_boxed_dyn_error () } }
};
}
