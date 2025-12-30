// Generated macro for impl_361 (impl)
macro_rules! Depcrate_sync_arcimpl_361 {
() => {
// Module: crate::sync::arc
// Provides: {"impl_361"}
// Dependencies: {}
impl < T : ? Sized > ops :: Deref for Arc < T > { type Target = T ; fn deref (& self) -> & T { & self . value } }
};
}
