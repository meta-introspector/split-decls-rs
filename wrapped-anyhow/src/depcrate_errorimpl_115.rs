// Generated macro for impl_115 (impl)
macro_rules! Depcrate_errorimpl_115 {
() => {
// Module: crate::error
// Provides: {"impl_115"}
// Dependencies: {}
# [cfg (any (feature = "std" , not (anyhow_no_core_error)))] impl From < Error > for Box < dyn StdError + 'static > { # [cold] fn from (error : Error) -> Self { error . into_boxed_dyn_error () } }
};
}
