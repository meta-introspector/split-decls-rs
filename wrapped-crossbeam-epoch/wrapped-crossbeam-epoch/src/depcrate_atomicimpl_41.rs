// Generated macro for impl_41 (impl)
macro_rules! Depcrate_atomicimpl_41 {
() => {
// Module: crate::atomic
// Provides: {"impl_41"}
// Dependencies: {}
impl < T : ? Sized + Pointable > Clone for Atomic < T > { # [doc = " Returns a copy of the atomic value."] # [doc = ""] # [doc = " Note that a `Relaxed` load is used here. If you need synchronization, use it with other"] # [doc = " atomics or fences."] fn clone (& self) -> Self { let data = self . data . load (Ordering :: Relaxed) ; Self :: from_ptr (data) } }
};
}
