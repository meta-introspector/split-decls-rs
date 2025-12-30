// Generated macro for impl_1979 (impl)
macro_rules! Depcrate_geometry_unit_complex_opsimpl_1979 {
() => {
// Module: crate::geometry::unit_complex_ops
// Provides: {"impl_1979"}
// Dependencies: {}
impl < T : SimdRealField > DivAssign < Rotation < T , 2 > > for UnitComplex < T > where T :: Element : SimdRealField , { # [inline] fn div_assign (& mut self , rhs : Rotation < T , 2 >) { * self = self . clone () / rhs } }
};
}
