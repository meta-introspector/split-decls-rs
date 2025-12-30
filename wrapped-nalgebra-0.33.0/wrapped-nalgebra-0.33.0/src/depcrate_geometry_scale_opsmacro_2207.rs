// Generated macro for macro_2207 (macro)
macro_rules! Depcrate_geometry_scale_opsmacro_2207 {
() => {
// Module: crate::geometry::scale_ops
// Provides: {"macro_2207"}
// Dependencies: {}
add_sub_impl ! (Mul , mul , ClosedMulAssign ; (Const < D >, U1) , (Const < D >, U1) -> (Const < D >, U1) const D ; for ; where ; self : Scale < T , D >, right : T , Output = Scale < T , D >; Scale :: from (self . vector * right) ;) ;
};
}
