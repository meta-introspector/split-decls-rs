// Generated macro for impl_208 (impl)
macro_rules! Depcrate_flatimpl_208 {
() => {
// Module: crate::flat
// Provides: {"impl_208"}
// Dependencies: {}
impl < Buffer , P : Pixel > GenericImage for ViewMut < Buffer , P > where Buffer : AsMut < [P :: Subpixel] > + AsRef < [P :: Subpixel] > , { fn get_pixel_mut (& mut self , x : u32 , y : u32) -> & mut Self :: Pixel { if ! self . inner . in_bounds (0 , x , y) { panic_pixel_out_of_bounds ((x , y) , self . dimensions ()) } let base_index = self . inner . in_bounds_index (0 , x , y) ; let channel_count = < P as Pixel > :: CHANNEL_COUNT as usize ; let pixel_range = base_index .. base_index + channel_count ; P :: from_slice_mut (& mut self . inner . samples . as_mut () [pixel_range]) } # [allow (deprecated)] fn put_pixel (& mut self , x : u32 , y : u32 , pixel : Self :: Pixel) { * self . get_pixel_mut (x , y) = pixel ; } # [allow (deprecated)] fn blend_pixel (& mut self , x : u32 , y : u32 , pixel : Self :: Pixel) { self . get_pixel_mut (x , y) . blend (& pixel) ; } }
};
}
