// Generated macro for op_assign_impl (macro)
macro_rules! Depcrate_geometry_point_opsop_assign_impl {
() => {
// Module: crate::geometry::point_ops
// Provides: {"op_assign_impl"}
// Dependencies: {}
macro_rules ! op_assign_impl (($ ($ TraitAssign : ident , $ method_assign : ident , $ bound : ident) ;* $ (;) *) => { $ (impl <'b , T , D1 : DimName , D2 : Dim , SB > $ TraitAssign <&'b Vector < T , D2 , SB >> for OPoint < T , D1 > where T : Scalar + $ bound , SB : Storage < T , D2 >, ShapeConstraint : SameNumberOfRows < D1 , D2 >, DefaultAllocator : Allocator < D1 > { # [inline] fn $ method_assign (& mut self , right : &'b Vector < T , D2 , SB >) { self . coords .$ method_assign (right) } } impl < T , D1 : DimName , D2 : Dim , SB > $ TraitAssign < Vector < T , D2 , SB >> for OPoint < T , D1 > where T : Scalar + $ bound , SB : Storage < T , D2 >, ShapeConstraint : SameNumberOfRows < D1 , D2 >, DefaultAllocator : Allocator < D1 > { # [inline] fn $ method_assign (& mut self , right : Vector < T , D2 , SB >) { self . coords .$ method_assign (right) } }) * }) ;
};
}
