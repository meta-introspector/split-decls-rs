// Generated macro for impl_821 (impl)
macro_rules! Depcrate_rc_retained_forwarding_implsimpl_821 {
() => {
// Module: crate::rc::retained_forwarding_impls
// Provides: {"impl_821"}
// Dependencies: {}
impl < T : ? Sized + AsRef < U > , U : ? Sized > AsRef < U > for Retained < T > { fn as_ref (& self) -> & U { (* * self) . as_ref () } }
};
}
