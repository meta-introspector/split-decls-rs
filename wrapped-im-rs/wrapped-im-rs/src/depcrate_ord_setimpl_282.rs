// Generated macro for impl_282 (impl)
macro_rules! Depcrate_ord_setimpl_282 {
() => {
// Module: crate::ord::set
// Provides: {"impl_282"}
// Dependencies: {}
impl < A : Ord + Clone > From < Vec < A > > for OrdSet < A > { fn from (vec : Vec < A >) -> Self { vec . into_iter () . collect () } }
};
}
