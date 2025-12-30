// Generated macro for impl_1981 (impl)
macro_rules! Depcrate_geometry_unit_complex_opsimpl_1981 {
() => {
// Module: crate::geometry::unit_complex_ops
// Provides: {"impl_1981"}
// Dependencies: {}
impl < T : SimdRealField > MulAssign < UnitComplex < T > > for Rotation < T , 2 > where T :: Element : SimdRealField , { # [inline] fn mul_assign (& mut self , rhs : UnitComplex < T >) { self . mul_assign (rhs . to_rotation_matrix ()) } }
};
}
