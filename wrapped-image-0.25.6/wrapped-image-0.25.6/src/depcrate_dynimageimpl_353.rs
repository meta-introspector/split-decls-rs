// Generated macro for impl_353 (impl)
macro_rules! Depcrate_dynimageimpl_353 {
() => {
// Module: crate::dynimage
// Provides: {"impl_353"}
// Dependencies: {}
# [allow (deprecated)] impl GenericImageView for DynamicImage { type Pixel = color :: Rgba < u8 > ; fn dimensions (& self) -> (u32 , u32) { dynamic_map ! (* self , ref p , p . dimensions ()) } fn get_pixel (& self , x : u32 , y : u32) -> color :: Rgba < u8 > { dynamic_map ! (* self , ref p , p . get_pixel (x , y) . to_rgba () . into_color ()) } }
};
}
