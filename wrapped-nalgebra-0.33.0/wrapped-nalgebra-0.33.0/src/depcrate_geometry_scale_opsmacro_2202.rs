// Generated macro for macro_2202 (macro)
macro_rules! Depcrate_geometry_scale_opsmacro_2202 {
() => {
// Module: crate::geometry::scale_ops
// Provides: {"macro_2202"}
// Dependencies: {}
add_sub_impl ! (Mul , mul , ClosedMulAssign ; (Const < D >, U1) , (Const < D >, U1) -> (Const < D >, U1) const D ; for ; where ; self : &'a Scale < T , D >, right : &'b Scale < T , D >, Output = Scale < T , D >; Scale :: from (self . vector . component_mul (& right . vector)) ; 'a , 'b) ;
};
}
