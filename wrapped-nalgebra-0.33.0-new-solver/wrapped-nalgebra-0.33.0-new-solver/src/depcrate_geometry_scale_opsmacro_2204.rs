// Generated macro for macro_2204 (macro)
macro_rules! Depcrate_geometry_scale_opsmacro_2204 {
() => {
// Module: crate::geometry::scale_ops
// Provides: {"macro_2204"}
// Dependencies: {}
add_sub_impl ! (Mul , mul , ClosedMulAssign ; (Const < D >, U1) , (Const < D >, U1) -> (Const < D >, U1) const D ; for ; where ; self : Scale < T , D >, right : &'b Scale < T , D >, Output = Scale < T , D >; Scale :: from (self . vector . component_mul (& right . vector)) ; 'b) ;
};
}
