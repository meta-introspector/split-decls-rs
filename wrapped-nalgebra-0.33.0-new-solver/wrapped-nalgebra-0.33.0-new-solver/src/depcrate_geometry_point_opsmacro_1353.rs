// Generated macro for macro_1353 (macro)
macro_rules! Depcrate_geometry_point_opsmacro_1353 {
() => {
// Module: crate::geometry::point_ops
// Provides: {"macro_1353"}
// Dependencies: {}
add_sub_impl ! (Sub , sub , ClosedSubAssign ; (D , U1) , (D , U1) -> (D , U1) const ; for D ; where D : DimName , DefaultAllocator : Allocator < D >; self : &'a OPoint < T , D >, right : &'b OPoint < T , D >, Output = OVector < T , D >; & self . coords - & right . coords ; 'a , 'b) ;
};
}
