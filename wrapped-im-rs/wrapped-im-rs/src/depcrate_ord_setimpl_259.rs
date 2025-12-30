// Generated macro for impl_259 (impl)
macro_rules! Depcrate_ord_setimpl_259 {
() => {
// Module: crate::ord::set
// Provides: {"impl_259"}
// Dependencies: {}
impl < A : Ord + Clone > Add for OrdSet < A > { type Output = OrdSet < A > ; fn add (self , other : Self) -> Self :: Output { self . union (other) } }
};
}
