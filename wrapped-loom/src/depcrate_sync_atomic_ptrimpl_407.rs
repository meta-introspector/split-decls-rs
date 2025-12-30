// Generated macro for impl_407 (impl)
macro_rules! Depcrate_sync_atomic_ptrimpl_407 {
() => {
// Module: crate::sync::atomic::ptr
// Provides: {"impl_407"}
// Dependencies: {}
impl < T > From < * mut T > for AtomicPtr < T > { fn from (p : * mut T) -> Self { Self :: new (p) } }
};
}
