// Generated macro for macro_1965 (macro)
macro_rules! Depcrate_geometry_unit_complex_opsmacro_1965 {
() => {
// Module: crate::geometry::unit_complex_ops
// Provides: {"macro_1965"}
// Dependencies: {}
complex_op_impl_all ! (Div , div ; ; self : Rotation < T , 2 >, rhs : UnitComplex < T >, Output = UnitComplex < T >; [val val] => & self / & rhs ; [ref val] => self / & rhs ; [val ref] => & self / rhs ; [ref ref] => # [allow (clippy :: suspicious_arithmetic_impl)] { UnitComplex :: from_rotation_matrix (self) * rhs . inverse () } ;) ;
};
}
