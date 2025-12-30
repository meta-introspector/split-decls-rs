// Generated macro for macro_1968 (macro)
macro_rules! Depcrate_geometry_unit_complex_opsmacro_1968 {
() => {
// Module: crate::geometry::unit_complex_ops
// Provides: {"macro_1968"}
// Dependencies: {}
complex_op_impl_all ! (Mul , mul ; S : Storage < T , Const < 2 >>; self : UnitComplex < T >, rhs : Unit < Vector < T , Const < 2 >, S >>, Output = Unit < Vector2 < T >>; [val val] => & self * & rhs ; [ref val] => self * & rhs ; [val ref] => & self * rhs ; [ref ref] => Unit :: new_unchecked (self * rhs . as_ref ()) ;) ;
};
}
