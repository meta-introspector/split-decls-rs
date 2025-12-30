// Generated macro for macro_1357 (macro)
macro_rules! Depcrate_geometry_point_opsmacro_1357 {
() => {
// Module: crate::geometry::point_ops
// Provides: {"macro_1357"}
// Dependencies: {}
add_sub_impl ! (Sub , sub , ClosedSubAssign ; (D1 , U1) , (D2 , U1) -> (D1 , U1) const ; for D1 , D2 , SB ; where D1 : DimName , D2 : Dim , SB : Storage < T , D2 >, DefaultAllocator : Allocator < D1 >; self : &'a OPoint < T , D1 >, right : &'b Vector < T , D2 , SB >, Output = OPoint < T , D1 >; Self :: Output :: from (& self . coords - right) ; 'a , 'b) ;
};
}
