// Generated macro for macro_1972 (macro)
macro_rules! Depcrate_geometry_unit_complex_opsmacro_1972 {
() => {
// Module: crate::geometry::unit_complex_ops
// Provides: {"macro_1972"}
// Dependencies: {}
complex_op_impl_all ! (Mul , mul ; ; self : Translation < T , 2 >, right : UnitComplex < T >, Output = Isometry < T , UnitComplex < T >, 2 >; [val val] => Isometry :: from_parts (self , right) ; [ref val] => Isometry :: from_parts (self . clone () , right) ; [val ref] => Isometry :: from_parts (self , right . clone ()) ; [ref ref] => Isometry :: from_parts (self . clone () , right . clone ()) ;) ;
};
}
