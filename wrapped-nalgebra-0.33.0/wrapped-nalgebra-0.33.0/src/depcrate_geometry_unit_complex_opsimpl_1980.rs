// Generated macro for impl_1980 (impl)
macro_rules! Depcrate_geometry_unit_complex_opsimpl_1980 {
() => {
// Module: crate::geometry::unit_complex_ops
// Provides: {"impl_1980"}
// Dependencies: {}
impl < 'b , T : SimdRealField > DivAssign < & 'b Rotation < T , 2 > > for UnitComplex < T > where T :: Element : SimdRealField , { # [inline] fn div_assign (& mut self , rhs : & 'b Rotation < T , 2 >) { * self = self . clone () / rhs } }
};
}
