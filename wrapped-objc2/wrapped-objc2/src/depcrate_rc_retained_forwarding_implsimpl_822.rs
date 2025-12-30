// Generated macro for impl_822 (impl)
macro_rules! Depcrate_rc_retained_forwarding_implsimpl_822 {
() => {
// Module: crate::rc::retained_forwarding_impls
// Provides: {"impl_822"}
// Dependencies: {}
# [cfg (feature = "std")] impl < T : ? Sized + Error > Error for Retained < T > { fn source (& self) -> Option < & (dyn Error + 'static) > { (* * self) . source () } }
};
}
