// Generated macro for impl_1974 (impl)
macro_rules! Depcrate_geometry_unit_complex_opsimpl_1974 {
() => {
// Module: crate::geometry::unit_complex_ops
// Provides: {"impl_1974"}
// Dependencies: {}
impl < 'b , T : SimdRealField > MulAssign < & 'b UnitComplex < T > > for UnitComplex < T > where T :: Element : SimdRealField , { # [inline] fn mul_assign (& mut self , rhs : & 'b UnitComplex < T >) { * self = self . clone () * rhs } }
};
}
