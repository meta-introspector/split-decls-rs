// Generated macro for impl_1555 (impl)
macro_rules! Depcrate_syncimpl_1555 {
() => {
// Module: crate::sync
// Provides: {"impl_1555"}
// Dependencies: {}
impl < T : ? Sized > Arc < T > { unsafe fn from_inner (ptr : NonNull < ArcInner < T > >) -> Self { unsafe { Self :: from_inner_in (ptr , Global) } } unsafe fn from_ptr (ptr : * mut ArcInner < T >) -> Self { unsafe { Self :: from_ptr_in (ptr , Global) } } }
};
}
