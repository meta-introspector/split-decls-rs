// Generated macro for impl_262 (impl)
macro_rules! Depcrate_ord_setimpl_262 {
() => {
// Module: crate::ord::set
// Provides: {"impl_262"}
// Dependencies: {}
impl < 'a , A : Ord + Clone > Mul for & 'a OrdSet < A > { type Output = OrdSet < A > ; fn mul (self , other : Self) -> Self :: Output { self . clone () . intersection (other . clone ()) } }
};
}
