// Generated macro for macro_2212 (macro)
macro_rules! Depcrate_geometry_scale_opsmacro_2212 {
() => {
// Module: crate::geometry::scale_ops
// Provides: {"macro_2212"}
// Dependencies: {}
add_sub_impl ! (Mul , mul , ClosedMulAssign ; (Const < D >, U1) , (Const < D >, U1) -> (Const < D >, U1) const D ; for ; where ; self : &'a Scale < T , D >, right : &'b SVector < T , D >, Output = SVector < T , D >; self . vector . component_mul (right) ; 'a , 'b) ;
};
}
