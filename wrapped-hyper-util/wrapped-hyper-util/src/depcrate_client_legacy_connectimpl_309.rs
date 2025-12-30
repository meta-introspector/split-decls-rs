// Generated macro for impl_309 (impl)
macro_rules! Depcrate_client_legacy_connectimpl_309 {
() => {
// Module: crate::client::legacy::connect
// Provides: {"impl_309"}
// Dependencies: {}
impl PoisonPill { pub (crate) fn healthy () -> Self { Self { poisoned : Arc :: new (AtomicBool :: new (false)) , } } pub (crate) fn poison (& self) { self . poisoned . store (true , Ordering :: Relaxed) } pub (crate) fn poisoned (& self) -> bool { self . poisoned . load (Ordering :: Relaxed) } }
};
}
