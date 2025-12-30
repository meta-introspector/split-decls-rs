// Generated macro for impl_1958 (impl)
macro_rules! Depcrate_geometry_unit_complex_opsimpl_1958 {
() => {
// Module: crate::geometry::unit_complex_ops
// Provides: {"impl_1958"}
// Dependencies: {}
impl < 'b , T : SimdRealField > Div < & 'b UnitComplex < T > > for UnitComplex < T > where T :: Element : SimdRealField , { type Output = Self ; # [inline] fn div (self , rhs : & 'b UnitComplex < T >) -> Self :: Output { # [allow (clippy :: suspicious_arithmetic_impl)] Unit :: new_unchecked (self . into_inner () * rhs . conjugate () . into_inner ()) } }
};
}
