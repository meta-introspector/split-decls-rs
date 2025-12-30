// Generated macro for impl_1984 (impl)
macro_rules! Depcrate_geometry_unit_complex_opsimpl_1984 {
() => {
// Module: crate::geometry::unit_complex_ops
// Provides: {"impl_1984"}
// Dependencies: {}
impl < 'b , T : SimdRealField > DivAssign < & 'b UnitComplex < T > > for Rotation < T , 2 > where T :: Element : SimdRealField , { # [inline] fn div_assign (& mut self , rhs : & 'b UnitComplex < T >) { self . div_assign (rhs . clone () . to_rotation_matrix ()) } }
};
}
