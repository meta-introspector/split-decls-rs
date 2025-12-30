// Generated macro for macro_1355 (macro)
macro_rules! Depcrate_geometry_point_opsmacro_1355 {
() => {
// Module: crate::geometry::point_ops
// Provides: {"macro_1355"}
// Dependencies: {}
add_sub_impl ! (Sub , sub , ClosedSubAssign ; (D , U1) , (D , U1) -> (D , U1) const ; for D ; where D : DimName , DefaultAllocator : Allocator < D >; self : OPoint < T , D >, right : &'b OPoint < T , D >, Output = OVector < T , D >; self . coords - & right . coords ; 'b) ;
};
}
