// Generated macro for macro_1361 (macro)
macro_rules! Depcrate_geometry_point_opsmacro_1361 {
() => {
// Module: crate::geometry::point_ops
// Provides: {"macro_1361"}
// Dependencies: {}
add_sub_impl ! (Add , add , ClosedAddAssign ; (D1 , U1) , (D2 , U1) -> (D1 , U1) const ; for D1 , D2 , SB ; where D1 : DimName , D2 : Dim , SB : Storage < T , D2 >, DefaultAllocator : Allocator < D1 >; self : &'a OPoint < T , D1 >, right : &'b Vector < T , D2 , SB >, Output = OPoint < T , D1 >; Self :: Output :: from (& self . coords + right) ; 'a , 'b) ;
};
}
