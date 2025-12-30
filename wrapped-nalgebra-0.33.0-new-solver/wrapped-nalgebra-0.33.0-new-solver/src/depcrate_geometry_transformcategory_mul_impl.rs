// Generated macro for category_mul_impl (macro)
macro_rules! Depcrate_geometry_transformcategory_mul_impl {
() => {
// Module: crate::geometry::transform
// Provides: {"category_mul_impl"}
// Dependencies: {}
macro_rules ! category_mul_impl (($ ($ a : ident * $ b : ident => $ c : ty) ;* $ (;) *) => { $ (impl TCategoryMul <$ a > for $ b { type Representative = $ c ; }) * }) ;
};
}
