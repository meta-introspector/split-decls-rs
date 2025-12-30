// Generated macro for impl_118 (impl)
macro_rules! Depcrateimpl_118 {
() => {
// Module: crate
// Provides: {"impl_118"}
// Dependencies: {}
impl < T : Clone + Num > One for Complex < T > { # [inline] fn one () -> Self { Self :: new (One :: one () , Zero :: zero ()) } # [inline] fn is_one (& self) -> bool { self . re . is_one () && self . im . is_zero () } # [inline] fn set_one (& mut self) { self . re . set_one () ; self . im . set_zero () ; } }
};
}
