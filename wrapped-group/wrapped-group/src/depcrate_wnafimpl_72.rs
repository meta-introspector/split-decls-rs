// Generated macro for impl_72 (impl)
macro_rules! Depcrate_wnafimpl_72 {
() => {
// Module: crate::wnaf
// Provides: {"impl_72"}
// Dependencies: {}
impl < G : Group , const WINDOW_SIZE : usize > Mul < & WnafScalar < G :: Scalar , WINDOW_SIZE > > for & WnafBase < G , WINDOW_SIZE > { type Output = G ; fn mul (self , rhs : & WnafScalar < G :: Scalar , WINDOW_SIZE >) -> Self :: Output { wnaf_exp (& self . table , & rhs . wnaf) } }
};
}
