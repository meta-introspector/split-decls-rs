// Generated macro for impl_915 (impl)
macro_rules! Depcrate_atomic_refimpl_915 {
() => {
// Module: crate::atomic_ref
// Provides: {"impl_915"}
// Dependencies: {}
impl < T : 'static > std :: ops :: Deref for AtomicRef < T > { type Target = T ; fn deref (& self) -> & Self :: Target { unsafe { & * self . 0 . load (Ordering :: SeqCst) } } }
};
}
