// Generated macro for macro_2211 (macro)
macro_rules! Depcrate_geometry_scale_opsmacro_2211 {
() => {
// Module: crate::geometry::scale_ops
// Provides: {"macro_2211"}
// Dependencies: {}
add_sub_impl ! (Mul , mul , ClosedMulAssign ; (Const < D >, U1) , (Const < D >, U1) -> (Const < D >, U1) const D ; for ; where ; self : Scale < T , D >, right : Point < T , D >, Output = Point < T , D >; Point :: from (self . vector . component_mul (& right . coords)) ;) ;
};
}
