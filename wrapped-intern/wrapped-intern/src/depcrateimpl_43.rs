// Generated macro for impl_43 (impl)
macro_rules! Depcrateimpl_43 {
() => {
// Module: crate
// Provides: {"impl_43"}
// Dependencies: {}
impl < T : Internable + ? Sized > Interned < T > { # [cold] fn drop_slow (& mut self) { let storage = T :: storage () . get () ; if Arc :: count (& self . arc) != 2 { return ; } storage . remove (& self . arc) ; } }
};
}
