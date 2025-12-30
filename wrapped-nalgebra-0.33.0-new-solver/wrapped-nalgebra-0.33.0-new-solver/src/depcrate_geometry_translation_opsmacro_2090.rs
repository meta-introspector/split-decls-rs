// Generated macro for macro_2090 (macro)
macro_rules! Depcrate_geometry_translation_opsmacro_2090 {
() => {
// Module: crate::geometry::translation_ops
// Provides: {"macro_2090"}
// Dependencies: {}
add_sub_impl ! (Mul , mul , ClosedAddAssign ; (Const < D >, U1) , (Const < D >, U1) -> (Const < D >, U1) const D ; for ; where ; self : Translation < T , D >, right : &'b Translation < T , D >, Output = Translation < T , D >; # [allow (clippy :: suspicious_arithmetic_impl)] { Translation :: from (self . vector + & right . vector) } ; 'b) ;
};
}
