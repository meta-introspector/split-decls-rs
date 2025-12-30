// Generated macro for impl_48 (impl)
macro_rules! Depcrate_errorimpl_48 {
() => {
// Module: crate::error
// Provides: {"impl_48"}
// Dependencies: {}
impl Deref for Report { type Target = dyn StdError + Send + Sync + 'static ; fn deref (& self) -> & Self :: Target { ErrorImpl :: error (self . inner . as_ref ()) } }
};
}
