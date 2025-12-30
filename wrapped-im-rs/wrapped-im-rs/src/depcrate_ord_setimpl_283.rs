// Generated macro for impl_283 (impl)
macro_rules! Depcrate_ord_setimpl_283 {
() => {
// Module: crate::ord::set
// Provides: {"impl_283"}
// Dependencies: {}
impl < 'a , A : Ord + Clone > From < & 'a Vec < A > > for OrdSet < A > { fn from (vec : & Vec < A >) -> Self { vec . iter () . cloned () . collect () } }
};
}
