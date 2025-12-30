// Generated macro for impl_117 (impl)
macro_rules! Depcrate_errorimpl_117 {
() => {
// Module: crate::error
// Provides: {"impl_117"}
// Dependencies: {}
# [cfg (any (feature = "std" , not (anyhow_no_core_error)))] impl AsRef < dyn StdError > for Error { fn as_ref (& self) -> & (dyn StdError + 'static) { & * * self } }
};
}
