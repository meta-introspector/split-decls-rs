// Generated macro for impl_116 (impl)
macro_rules! Depcrate_errorimpl_116 {
() => {
// Module: crate::error
// Provides: {"impl_116"}
// Dependencies: {}
# [cfg (any (feature = "std" , not (anyhow_no_core_error)))] impl AsRef < dyn StdError + Send + Sync > for Error { fn as_ref (& self) -> & (dyn StdError + Send + Sync + 'static) { & * * self } }
};
}
