// Generated macro for macro_2217 (macro)
macro_rules! Depcrate_geometry_scale_opsmacro_2217 {
() => {
// Module: crate::geometry::scale_ops
// Provides: {"macro_2217"}
// Dependencies: {}
add_sub_assign_impl ! (MulAssign , mul_assign , ClosedMulAssign ; const D ; self : Scale < T , D >, right : Scale < T , D >; self . vector . component_mul_assign (& right . vector) ;) ;
};
}
