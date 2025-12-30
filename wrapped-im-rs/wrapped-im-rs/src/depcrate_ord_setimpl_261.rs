// Generated macro for impl_261 (impl)
macro_rules! Depcrate_ord_setimpl_261 {
() => {
// Module: crate::ord::set
// Provides: {"impl_261"}
// Dependencies: {}
impl < A : Ord + Clone > Mul for OrdSet < A > { type Output = OrdSet < A > ; fn mul (self , other : Self) -> Self :: Output { self . intersection (other) } }
};
}
