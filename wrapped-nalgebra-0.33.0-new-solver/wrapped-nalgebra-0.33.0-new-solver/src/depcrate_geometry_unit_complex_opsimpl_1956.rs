// Generated macro for impl_1956 (impl)
macro_rules! Depcrate_geometry_unit_complex_opsimpl_1956 {
() => {
// Module: crate::geometry::unit_complex_ops
// Provides: {"impl_1956"}
// Dependencies: {}
impl < T : SimdRealField > Div < Self > for UnitComplex < T > where T :: Element : SimdRealField , { type Output = Self ; # [inline] fn div (self , rhs : Self) -> Self :: Output { # [allow (clippy :: suspicious_arithmetic_impl)] Unit :: new_unchecked (self . into_inner () * rhs . conjugate () . into_inner ()) } }
};
}
