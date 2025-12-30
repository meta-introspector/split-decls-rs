// Generated macro for macro_1963 (macro)
macro_rules! Depcrate_geometry_unit_complex_opsmacro_1963 {
() => {
// Module: crate::geometry::unit_complex_ops
// Provides: {"macro_1963"}
// Dependencies: {}
complex_op_impl_all ! (Div , div ; ; self : UnitComplex < T >, rhs : Rotation < T , 2 >, Output = UnitComplex < T >; [val val] => & self / & rhs ; [ref val] => self / & rhs ; [val ref] => & self / rhs ; [ref ref] => # [allow (clippy :: suspicious_arithmetic_impl)] { self * UnitComplex :: from_rotation_matrix (rhs) . inverse () } ;) ;
};
}
