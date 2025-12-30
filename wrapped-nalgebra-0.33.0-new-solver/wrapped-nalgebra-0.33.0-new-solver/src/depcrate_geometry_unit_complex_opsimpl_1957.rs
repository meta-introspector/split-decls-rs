// Generated macro for impl_1957 (impl)
macro_rules! Depcrate_geometry_unit_complex_opsimpl_1957 {
() => {
// Module: crate::geometry::unit_complex_ops
// Provides: {"impl_1957"}
// Dependencies: {}
impl < 'a , T : SimdRealField > Div < UnitComplex < T > > for & 'a UnitComplex < T > where T :: Element : SimdRealField , { type Output = UnitComplex < T > ; # [inline] fn div (self , rhs : UnitComplex < T >) -> Self :: Output { # [allow (clippy :: suspicious_arithmetic_impl)] Unit :: new_unchecked (self . complex () * rhs . conjugate () . into_inner ()) } }
};
}
