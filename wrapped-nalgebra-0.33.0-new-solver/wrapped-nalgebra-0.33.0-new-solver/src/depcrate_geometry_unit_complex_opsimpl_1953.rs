// Generated macro for impl_1953 (impl)
macro_rules! Depcrate_geometry_unit_complex_opsimpl_1953 {
() => {
// Module: crate::geometry::unit_complex_ops
// Provides: {"impl_1953"}
// Dependencies: {}
impl < 'a , T : SimdRealField > Mul < UnitComplex < T > > for & 'a UnitComplex < T > where T :: Element : SimdRealField , { type Output = UnitComplex < T > ; # [inline] fn mul (self , rhs : UnitComplex < T >) -> Self :: Output { Unit :: new_unchecked (self . complex () * rhs . into_inner ()) } }
};
}
