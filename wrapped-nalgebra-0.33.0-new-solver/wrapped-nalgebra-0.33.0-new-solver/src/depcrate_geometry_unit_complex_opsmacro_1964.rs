// Generated macro for macro_1964 (macro)
macro_rules! Depcrate_geometry_unit_complex_opsmacro_1964 {
() => {
// Module: crate::geometry::unit_complex_ops
// Provides: {"macro_1964"}
// Dependencies: {}
complex_op_impl_all ! (Mul , mul ; ; self : Rotation < T , 2 >, rhs : UnitComplex < T >, Output = UnitComplex < T >; [val val] => & self * & rhs ; [ref val] => self * & rhs ; [val ref] => & self * rhs ; [ref ref] => UnitComplex :: from_rotation_matrix (self) * rhs ;) ;
};
}
