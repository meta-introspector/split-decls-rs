// Generated macro for macro_1969 (macro)
macro_rules! Depcrate_geometry_unit_complex_opsmacro_1969 {
() => {
// Module: crate::geometry::unit_complex_ops
// Provides: {"macro_1969"}
// Dependencies: {}
complex_op_impl_all ! (Mul , mul ; ; self : UnitComplex < T >, rhs : Isometry < T , UnitComplex < T >, 2 >, Output = Isometry < T , UnitComplex < T >, 2 >; [val val] => & self * & rhs ; [ref val] => self * & rhs ; [val ref] => & self * rhs ; [ref ref] => { let shift = self * & rhs . translation . vector ; Isometry :: from_parts (Translation :: from (shift) , self * & rhs . rotation) } ;) ;
};
}
