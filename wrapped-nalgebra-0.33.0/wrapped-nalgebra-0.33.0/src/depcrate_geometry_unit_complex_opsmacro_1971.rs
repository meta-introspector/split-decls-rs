// Generated macro for macro_1971 (macro)
macro_rules! Depcrate_geometry_unit_complex_opsmacro_1971 {
() => {
// Module: crate::geometry::unit_complex_ops
// Provides: {"macro_1971"}
// Dependencies: {}
complex_op_impl_all ! (Mul , mul ; ; self : UnitComplex < T >, rhs : Translation < T , 2 >, Output = Isometry < T , UnitComplex < T >, 2 >; [val val] => Isometry :: from_parts (Translation :: from (& self * rhs . vector) , self) ; [ref val] => Isometry :: from_parts (Translation :: from (self * rhs . vector) , self . clone ()) ; [val ref] => Isometry :: from_parts (Translation :: from (& self * & rhs . vector) , self) ; [ref ref] => Isometry :: from_parts (Translation :: from (self * & rhs . vector) , self . clone ()) ;) ;
};
}
