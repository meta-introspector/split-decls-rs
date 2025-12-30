// Generated macro for impl_1952 (impl)
macro_rules! Depcrate_geometry_unit_complex_opsimpl_1952 {
() => {
// Module: crate::geometry::unit_complex_ops
// Provides: {"impl_1952"}
// Dependencies: {}
impl < T : SimdRealField > Mul < Self > for UnitComplex < T > { type Output = Self ; # [inline] fn mul (self , rhs : Self) -> Self { Unit :: new_unchecked (self . into_inner () * rhs . into_inner ()) } }
};
}
