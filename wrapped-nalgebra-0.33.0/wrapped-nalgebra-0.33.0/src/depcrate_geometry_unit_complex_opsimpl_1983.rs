// Generated macro for impl_1983 (impl)
macro_rules! Depcrate_geometry_unit_complex_opsimpl_1983 {
() => {
// Module: crate::geometry::unit_complex_ops
// Provides: {"impl_1983"}
// Dependencies: {}
impl < T : SimdRealField > DivAssign < UnitComplex < T > > for Rotation < T , 2 > where T :: Element : SimdRealField , { # [inline] fn div_assign (& mut self , rhs : UnitComplex < T >) { self . div_assign (rhs . to_rotation_matrix ()) } }
};
}
