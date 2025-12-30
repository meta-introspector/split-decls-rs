// Generated macro for macro_1966 (macro)
macro_rules! Depcrate_geometry_unit_complex_opsmacro_1966 {
() => {
// Module: crate::geometry::unit_complex_ops
// Provides: {"macro_1966"}
// Dependencies: {}
complex_op_impl_all ! (Mul , mul ; ; self : UnitComplex < T >, rhs : Point2 < T >, Output = Point2 < T >; [val val] => & self * & rhs ; [ref val] => self * & rhs ; [val ref] => & self * rhs ; [ref ref] => Point2 :: from (self * & rhs . coords) ;) ;
};
}
