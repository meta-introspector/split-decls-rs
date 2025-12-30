// Generated macro for impl_1959 (impl)
macro_rules! Depcrate_geometry_unit_complex_opsimpl_1959 {
() => {
// Module: crate::geometry::unit_complex_ops
// Provides: {"impl_1959"}
// Dependencies: {}
impl < 'a , 'b , T : SimdRealField > Div < & 'b UnitComplex < T > > for & 'a UnitComplex < T > where T :: Element : SimdRealField , { type Output = UnitComplex < T > ; # [inline] fn div (self , rhs : & 'b UnitComplex < T >) -> Self :: Output { # [allow (clippy :: suspicious_arithmetic_impl)] Unit :: new_unchecked (self . complex () * rhs . conjugate () . into_inner ()) } }
};
}
