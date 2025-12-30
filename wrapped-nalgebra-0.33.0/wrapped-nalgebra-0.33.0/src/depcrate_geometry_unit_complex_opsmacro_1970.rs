// Generated macro for macro_1970 (macro)
macro_rules! Depcrate_geometry_unit_complex_opsmacro_1970 {
() => {
// Module: crate::geometry::unit_complex_ops
// Provides: {"macro_1970"}
// Dependencies: {}
complex_op_impl_all ! (Mul , mul ; ; self : UnitComplex < T >, rhs : Similarity < T , UnitComplex < T >, 2 >, Output = Similarity < T , UnitComplex < T >, 2 >; [val val] => & self * & rhs ; [ref val] => self * & rhs ; [val ref] => & self * rhs ; [ref ref] => Similarity :: from_isometry (self * & rhs . isometry , rhs . scaling ()) ;) ;
};
}
