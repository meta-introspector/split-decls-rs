// Generated macro for macro_2096 (macro)
macro_rules! Depcrate_geometry_translation_opsmacro_2096 {
() => {
// Module: crate::geometry::translation_ops
// Provides: {"macro_2096"}
// Dependencies: {}
add_sub_impl ! (Mul , mul , ClosedAddAssign ; (Const < D >, U1) , (Const < D >, U1) -> (Const < D >, U1) const D ; for ; where ; self : &'a Translation < T , D >, right : &'b Point < T , D >, Output = Point < T , D >; # [allow (clippy :: suspicious_arithmetic_impl)] { right + & self . vector } ; 'a , 'b) ;
};
}
