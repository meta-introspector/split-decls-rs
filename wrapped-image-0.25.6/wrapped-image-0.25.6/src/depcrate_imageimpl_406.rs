// Generated macro for impl_406 (impl)
macro_rules! Depcrate_imageimpl_406 {
() => {
// Module: crate::image
// Provides: {"impl_406"}
// Dependencies: {}
# [allow (deprecated)] impl < I > GenericImage for SubImageInner < I > where I : DerefMut , I :: Target : GenericImage + Sized , { fn get_pixel_mut (& mut self , x : u32 , y : u32) -> & mut Self :: Pixel { self . image . get_pixel_mut (x + self . xoffset , y + self . yoffset) } fn put_pixel (& mut self , x : u32 , y : u32 , pixel : Self :: Pixel) { self . image . put_pixel (x + self . xoffset , y + self . yoffset , pixel) ; } # [doc = " DEPRECATED: This method will be removed. Blend the pixel directly instead."] fn blend_pixel (& mut self , x : u32 , y : u32 , pixel : Self :: Pixel) { self . image . blend_pixel (x + self . xoffset , y + self . yoffset , pixel) ; } }
};
}
