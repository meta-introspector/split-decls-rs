// Generated macro for impl_31 (impl)
macro_rules! Depcrate_local_poolimpl_31 {
() => {
// Module: crate::local_pool
// Provides: {"impl_31"}
// Dependencies: {}
impl < S : Stream + Unpin > Deref for BlockingStream < S > { type Target = S ; fn deref (& self) -> & Self :: Target { & self . stream } }
};
}
