// Generated macro for macro_2206 (macro)
macro_rules! Depcrate_geometry_scale_opsmacro_2206 {
() => {
// Module: crate::geometry::scale_ops
// Provides: {"macro_2206"}
// Dependencies: {}
add_sub_impl ! (Mul , mul , ClosedMulAssign ; (Const < D >, U1) , (Const < D >, U1) -> (Const < D >, U1) const D ; for ; where ; self : &'a Scale < T , D >, right : T , Output = Scale < T , D >; Scale :: from (& self . vector * right) ; 'a) ;
};
}
