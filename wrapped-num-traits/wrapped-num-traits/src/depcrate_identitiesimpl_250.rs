// Generated macro for impl_250 (impl)
macro_rules! Depcrate_identitiesimpl_250 {
() => {
// Module: crate::identities
// Provides: {"impl_250"}
// Dependencies: {}
impl < T : Zero > Zero for Wrapping < T > where Wrapping < T > : Add < Output = Wrapping < T > > , { fn is_zero (& self) -> bool { self . 0 . is_zero () } fn set_zero (& mut self) { self . 0 . set_zero () ; } fn zero () -> Self { Wrapping (T :: zero ()) } }
};
}
