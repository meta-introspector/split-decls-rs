// Generated macro for macro_2203 (macro)
macro_rules! Depcrate_geometry_scale_opsmacro_2203 {
() => {
// Module: crate::geometry::scale_ops
// Provides: {"macro_2203"}
// Dependencies: {}
add_sub_impl ! (Mul , mul , ClosedMulAssign ; (Const < D >, U1) , (Const < D >, U1) -> (Const < D >, U1) const D ; for ; where ; self : &'a Scale < T , D >, right : Scale < T , D >, Output = Scale < T , D >; Scale :: from (self . vector . component_mul (& right . vector)) ; 'a) ;
};
}
