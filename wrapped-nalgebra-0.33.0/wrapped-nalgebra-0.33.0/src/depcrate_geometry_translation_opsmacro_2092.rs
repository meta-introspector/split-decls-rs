// Generated macro for macro_2092 (macro)
macro_rules! Depcrate_geometry_translation_opsmacro_2092 {
() => {
// Module: crate::geometry::translation_ops
// Provides: {"macro_2092"}
// Dependencies: {}
add_sub_impl ! (Div , div , ClosedSubAssign ; (Const < D >, U1) , (Const < D >, U1) -> (Const < D >, U1) const D ; for ; where ; self : &'a Translation < T , D >, right : &'b Translation < T , D >, Output = Translation < T , D >; # [allow (clippy :: suspicious_arithmetic_impl)] { Translation :: from (& self . vector - & right . vector) } ; 'a , 'b) ;
};
}
