// Generated macro for impl_803 (impl)
macro_rules! Depcrate_rc_retained_forwarding_implsimpl_803 {
() => {
// Module: crate::rc::retained_forwarding_impls
// Provides: {"impl_803"}
// Dependencies: {}
impl < T : ? Sized + PartialEq < U > , U : ? Sized > PartialEq < Retained < U > > for Retained < T > { # [inline] fn eq (& self , other : & Retained < U >) -> bool { (* * self) . eq (& * * other) } # [inline] # [allow (clippy :: partialeq_ne_impl)] fn ne (& self , other : & Retained < U >) -> bool { (* * self) . ne (& * * other) } }
};
}
