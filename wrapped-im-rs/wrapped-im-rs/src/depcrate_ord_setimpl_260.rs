// Generated macro for impl_260 (impl)
macro_rules! Depcrate_ord_setimpl_260 {
() => {
// Module: crate::ord::set
// Provides: {"impl_260"}
// Dependencies: {}
impl < 'a , A : Ord + Clone > Add for & 'a OrdSet < A > { type Output = OrdSet < A > ; fn add (self , other : Self) -> Self :: Output { self . clone () . union (other . clone ()) } }
};
}
