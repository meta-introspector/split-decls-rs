// Generated macro for impl_32 (impl)
macro_rules! Depcrate_local_poolimpl_32 {
() => {
// Module: crate::local_pool
// Provides: {"impl_32"}
// Dependencies: {}
impl < S : Stream + Unpin > DerefMut for BlockingStream < S > { fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . stream } }
};
}
