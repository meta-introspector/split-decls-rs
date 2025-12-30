// Generated macro for left_scalar_mul_impl (macro)
macro_rules! Depcrate_geometry_point_opsleft_scalar_mul_impl {
() => {
// Module: crate::geometry::point_ops
// Provides: {"left_scalar_mul_impl"}
// Dependencies: {}
macro_rules ! left_scalar_mul_impl (($ ($ T : ty) ,* $ (,) *) => { $ (impl < D : DimName > Mul < OPoint <$ T , D >> for $ T where DefaultAllocator : Allocator < D > { type Output = OPoint <$ T , D >; # [inline] fn mul (self , right : OPoint <$ T , D >) -> Self :: Output { OPoint :: from (self * right . coords) } } impl <'b , D : DimName > Mul <&'b OPoint <$ T , D >> for $ T where DefaultAllocator : Allocator < D > { type Output = OPoint <$ T , D >; # [inline] fn mul (self , right : &'b OPoint <$ T , D >) -> Self :: Output { OPoint :: from (self * & right . coords) } }) * }) ;
};
}
