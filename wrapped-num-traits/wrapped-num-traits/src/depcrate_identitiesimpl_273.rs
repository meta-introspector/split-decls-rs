// Generated macro for impl_273 (impl)
macro_rules! Depcrate_identitiesimpl_273 {
() => {
// Module: crate::identities
// Provides: {"impl_273"}
// Dependencies: {}
# [cfg (has_num_saturating)] impl < T : One > One for Saturating < T > where Saturating < T > : Mul < Output = Saturating < T > > , { fn set_one (& mut self) { self . 0 . set_one () ; } fn one () -> Self { Saturating (T :: one ()) } }
};
}
