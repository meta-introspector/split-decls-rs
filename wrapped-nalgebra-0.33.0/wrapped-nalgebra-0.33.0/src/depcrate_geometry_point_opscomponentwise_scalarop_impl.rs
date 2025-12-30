// Generated macro for componentwise_scalarop_impl (macro)
macro_rules! Depcrate_geometry_point_opscomponentwise_scalarop_impl {
() => {
// Module: crate::geometry::point_ops
// Provides: {"componentwise_scalarop_impl"}
// Dependencies: {}
macro_rules ! componentwise_scalarop_impl (($ Trait : ident , $ method : ident , $ bound : ident ; $ TraitAssign : ident , $ method_assign : ident) => { impl < T : Scalar + $ bound , D : DimName > $ Trait < T > for OPoint < T , D > where DefaultAllocator : Allocator < D > { type Output = OPoint < T , D >; # [inline] fn $ method (self , right : T) -> Self :: Output { OPoint :: from (self . coords .$ method (right)) } } impl <'a , T : Scalar + $ bound , D : DimName > $ Trait < T > for &'a OPoint < T , D > where DefaultAllocator : Allocator < D > { type Output = OPoint < T , D >; # [inline] fn $ method (self , right : T) -> Self :: Output { OPoint :: from ((& self . coords) .$ method (right)) } } impl < T : Scalar + $ bound , D : DimName > $ TraitAssign < T > for OPoint < T , D > where DefaultAllocator : Allocator < D > { # [inline] fn $ method_assign (& mut self , right : T) { self . coords .$ method_assign (right) } } }) ;
};
}
