// Generated macro for macro_1962 (macro)
macro_rules! Depcrate_geometry_unit_complex_opsmacro_1962 {
() => {
// Module: crate::geometry::unit_complex_ops
// Provides: {"macro_1962"}
// Dependencies: {}
complex_op_impl_all ! (Mul , mul ; ; self : UnitComplex < T >, rhs : Rotation < T , 2 >, Output = UnitComplex < T >; [val val] => & self * & rhs ; [ref val] => self * & rhs ; [val ref] => & self * rhs ; [ref ref] => self * UnitComplex :: from_rotation_matrix (rhs) ;) ;
};
}
