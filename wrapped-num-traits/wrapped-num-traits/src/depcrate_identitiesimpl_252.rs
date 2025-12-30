// Generated macro for impl_252 (impl)
macro_rules! Depcrate_identitiesimpl_252 {
() => {
// Module: crate::identities
// Provides: {"impl_252"}
// Dependencies: {}
# [cfg (has_num_saturating)] impl < T : Zero > Zero for Saturating < T > where Saturating < T > : Add < Output = Saturating < T > > , { fn is_zero (& self) -> bool { self . 0 . is_zero () } fn set_zero (& mut self) { self . 0 . set_zero () ; } fn zero () -> Self { Saturating (T :: zero ()) } }
};
}
