// Generated macro for impl_1976 (impl)
macro_rules! Depcrate_geometry_unit_complex_opsimpl_1976 {
() => {
// Module: crate::geometry::unit_complex_ops
// Provides: {"impl_1976"}
// Dependencies: {}
impl < 'b , T : SimdRealField > DivAssign < & 'b UnitComplex < T > > for UnitComplex < T > where T :: Element : SimdRealField , { # [inline] fn div_assign (& mut self , rhs : & 'b UnitComplex < T >) { * self = self . clone () / rhs } }
};
}
