// Generated macro for impl_1975 (impl)
macro_rules! Depcrate_geometry_unit_complex_opsimpl_1975 {
() => {
// Module: crate::geometry::unit_complex_ops
// Provides: {"impl_1975"}
// Dependencies: {}
impl < T : SimdRealField > DivAssign < UnitComplex < T > > for UnitComplex < T > where T :: Element : SimdRealField , { # [inline] fn div_assign (& mut self , rhs : UnitComplex < T >) { * self = self . clone () / rhs } }
};
}
