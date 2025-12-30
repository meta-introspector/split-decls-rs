// Generated macro for impl_807 (impl)
macro_rules! Depcrate_rc_retained_forwarding_implsimpl_807 {
() => {
// Module: crate::rc::retained_forwarding_impls
// Provides: {"impl_807"}
// Dependencies: {}
impl < T : hash :: Hash + ? Sized > hash :: Hash for Retained < T > { fn hash < H : hash :: Hasher > (& self , state : & mut H) { (* * self) . hash (state) ; } }
};
}
