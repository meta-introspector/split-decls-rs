// Generated macro for impl_1978 (impl)
macro_rules! Depcrate_geometry_unit_complex_opsimpl_1978 {
() => {
// Module: crate::geometry::unit_complex_ops
// Provides: {"impl_1978"}
// Dependencies: {}
impl < 'b , T : SimdRealField > MulAssign < & 'b Rotation < T , 2 > > for UnitComplex < T > where T :: Element : SimdRealField , { # [inline] fn mul_assign (& mut self , rhs : & 'b Rotation < T , 2 >) { * self = self . clone () * rhs } }
};
}
