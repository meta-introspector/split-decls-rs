// Generated macro for impl_83 (impl)
macro_rules! Depcrate_errorimpl_83 {
() => {
// Module: crate::error
// Provides: {"impl_83"}
// Dependencies: {}
# [cfg (any (feature = "std" , not (anyhow_no_core_error)))] impl Deref for Error { type Target = dyn StdError + Send + Sync + 'static ; fn deref (& self) -> & Self :: Target { unsafe { ErrorImpl :: error (self . inner . by_ref ()) } } }
};
}
