// Generated macro for impl_271 (impl)
macro_rules! Depcrate_identitiesimpl_271 {
() => {
// Module: crate::identities
// Provides: {"impl_271"}
// Dependencies: {}
impl < T : One > One for Wrapping < T > where Wrapping < T > : Mul < Output = Wrapping < T > > , { fn set_one (& mut self) { self . 0 . set_one () ; } fn one () -> Self { Wrapping (T :: one ()) } }
};
}
