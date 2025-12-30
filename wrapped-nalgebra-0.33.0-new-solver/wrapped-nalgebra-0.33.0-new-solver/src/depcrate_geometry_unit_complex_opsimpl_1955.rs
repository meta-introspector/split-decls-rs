// Generated macro for impl_1955 (impl)
macro_rules! Depcrate_geometry_unit_complex_opsimpl_1955 {
() => {
// Module: crate::geometry::unit_complex_ops
// Provides: {"impl_1955"}
// Dependencies: {}
impl < 'a , 'b , T : SimdRealField > Mul < & 'b UnitComplex < T > > for & 'a UnitComplex < T > where T :: Element : SimdRealField , { type Output = UnitComplex < T > ; # [inline] fn mul (self , rhs : & 'b UnitComplex < T >) -> Self :: Output { Unit :: new_unchecked (self . complex () * rhs . as_ref ()) } }
};
}
