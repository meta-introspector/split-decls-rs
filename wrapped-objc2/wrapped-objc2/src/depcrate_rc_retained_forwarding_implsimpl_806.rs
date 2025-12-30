// Generated macro for impl_806 (impl)
macro_rules! Depcrate_rc_retained_forwarding_implsimpl_806 {
() => {
// Module: crate::rc::retained_forwarding_impls
// Provides: {"impl_806"}
// Dependencies: {}
impl < T : Ord + ? Sized > Ord for Retained < T > { # [inline] fn cmp (& self , other : & Self) -> Ordering { (* * self) . cmp (& * * other) } }
};
}
