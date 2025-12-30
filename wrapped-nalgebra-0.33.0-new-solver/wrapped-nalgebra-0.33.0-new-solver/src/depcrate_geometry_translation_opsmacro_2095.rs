// Generated macro for macro_2095 (macro)
macro_rules! Depcrate_geometry_translation_opsmacro_2095 {
() => {
// Module: crate::geometry::translation_ops
// Provides: {"macro_2095"}
// Dependencies: {}
add_sub_impl ! (Div , div , ClosedSubAssign ; (Const < D >, U1) , (Const < D >, U1) -> (Const < D >, U1) const D ; for ; where ; self : Translation < T , D >, right : Translation < T , D >, Output = Translation < T , D >; # [allow (clippy :: suspicious_arithmetic_impl)] { Translation :: from (self . vector - right . vector) } ;) ;
};
}
