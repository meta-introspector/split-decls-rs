// Generated macro for impl_115 (impl)
macro_rules! Depcrateimpl_115 {
() => {
// Module: crate
// Provides: {"impl_115"}
// Dependencies: {}
impl < T : Clone + Num > Zero for Complex < T > { # [inline] fn zero () -> Self { Self :: new (Zero :: zero () , Zero :: zero ()) } # [inline] fn is_zero (& self) -> bool { self . re . is_zero () && self . im . is_zero () } # [inline] fn set_zero (& mut self) { self . re . set_zero () ; self . im . set_zero () ; } }
};
}
