// Generated macro for impl_207 (impl)
macro_rules! Depcrate_flatimpl_207 {
() => {
// Module: crate::flat
// Provides: {"impl_207"}
// Dependencies: {}
impl < Buffer , P : Pixel > GenericImageView for ViewMut < Buffer , P > where Buffer : AsMut < [P :: Subpixel] > + AsRef < [P :: Subpixel] > , { type Pixel = P ; fn dimensions (& self) -> (u32 , u32) { (self . inner . layout . width , self . inner . layout . height) } fn get_pixel (& self , x : u32 , y : u32) -> Self :: Pixel { if ! self . inner . in_bounds (0 , x , y) { panic_pixel_out_of_bounds ((x , y) , self . dimensions ()) } let image = self . inner . samples . as_ref () ; let base_index = self . inner . in_bounds_index (0 , x , y) ; let channels = P :: CHANNEL_COUNT as usize ; let mut buffer = [Zero :: zero () ; 256] ; buffer . iter_mut () . enumerate () . take (channels) . for_each (| (c , to) | { let index = base_index + c * self . inner . layout . channel_stride ; * to = image [index] ; }) ; * P :: from_slice (& buffer [.. channels]) } }
};
}
