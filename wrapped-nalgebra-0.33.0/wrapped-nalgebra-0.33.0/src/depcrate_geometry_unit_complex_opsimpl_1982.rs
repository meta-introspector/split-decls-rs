// Generated macro for impl_1982 (impl)
macro_rules! Depcrate_geometry_unit_complex_opsimpl_1982 {
() => {
// Module: crate::geometry::unit_complex_ops
// Provides: {"impl_1982"}
// Dependencies: {}
impl < 'b , T : SimdRealField > MulAssign < & 'b UnitComplex < T > > for Rotation < T , 2 > where T :: Element : SimdRealField , { # [inline] fn mul_assign (& mut self , rhs : & 'b UnitComplex < T >) { self . mul_assign (rhs . clone () . to_rotation_matrix ()) } }
};
}
