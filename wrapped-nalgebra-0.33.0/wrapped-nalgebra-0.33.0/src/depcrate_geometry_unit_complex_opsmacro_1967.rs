// Generated macro for macro_1967 (macro)
macro_rules! Depcrate_geometry_unit_complex_opsmacro_1967 {
() => {
// Module: crate::geometry::unit_complex_ops
// Provides: {"macro_1967"}
// Dependencies: {}
complex_op_impl_all ! (Mul , mul ; S : Storage < T , Const < 2 >>; self : UnitComplex < T >, rhs : Vector < T , Const < 2 >, S >, Output = Vector2 < T >; [val val] => & self * & rhs ; [ref val] => self * & rhs ; [val ref] => & self * rhs ; [ref ref] => { let i = self . as_ref () . im . clone () ; let r = self . as_ref () . re . clone () ; Vector2 :: new (r . clone () * rhs [0] . clone () - i . clone () * rhs [1] . clone () , i * rhs [0] . clone () + r * rhs [1] . clone ()) } ;) ;
};
}
