// Generated macro for impl_1977 (impl)
macro_rules! Depcrate_geometry_unit_complex_opsimpl_1977 {
() => {
// Module: crate::geometry::unit_complex_ops
// Provides: {"impl_1977"}
// Dependencies: {}
impl < T : SimdRealField > MulAssign < Rotation < T , 2 > > for UnitComplex < T > where T :: Element : SimdRealField , { # [inline] fn mul_assign (& mut self , rhs : Rotation < T , 2 >) { * self = self . clone () * rhs } }
};
}
