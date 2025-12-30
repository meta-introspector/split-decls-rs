// Generated macro for impl_1954 (impl)
macro_rules! Depcrate_geometry_unit_complex_opsimpl_1954 {
() => {
// Module: crate::geometry::unit_complex_ops
// Provides: {"impl_1954"}
// Dependencies: {}
impl < 'b , T : SimdRealField > Mul < & 'b UnitComplex < T > > for UnitComplex < T > where T :: Element : SimdRealField , { type Output = Self ; # [inline] fn mul (self , rhs : & 'b UnitComplex < T >) -> Self :: Output { Unit :: new_unchecked (self . into_inner () * rhs . as_ref ()) } }
};
}
