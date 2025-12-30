// Generated macro for impl_1973 (impl)
macro_rules! Depcrate_geometry_unit_complex_opsimpl_1973 {
() => {
// Module: crate::geometry::unit_complex_ops
// Provides: {"impl_1973"}
// Dependencies: {}
impl < T : SimdRealField > MulAssign < UnitComplex < T > > for UnitComplex < T > where T :: Element : SimdRealField , { # [inline] fn mul_assign (& mut self , rhs : UnitComplex < T >) { * self = self . clone () * rhs } }
};
}
