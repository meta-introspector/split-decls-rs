// Generated macro for impl_84 (impl)
macro_rules! Depcrate_errorimpl_84 {
() => {
// Module: crate::error
// Provides: {"impl_84"}
// Dependencies: {}
# [cfg (any (feature = "std" , not (anyhow_no_core_error)))] impl DerefMut for Error { fn deref_mut (& mut self) -> & mut Self :: Target { unsafe { ErrorImpl :: error_mut (self . inner . by_mut ()) } } }
};
}
