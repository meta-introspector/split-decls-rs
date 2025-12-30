// Generated macro for impl_405 (impl)
macro_rules! Depcrate_imageimpl_405 {
() => {
// Module: crate::image
// Provides: {"impl_405"}
// Dependencies: {}
# [allow (deprecated)] impl < I > GenericImageView for SubImageInner < I > where I : Deref , I :: Target : GenericImageView , { type Pixel = DerefPixel < I > ; fn dimensions (& self) -> (u32 , u32) { (self . xstride , self . ystride) } fn get_pixel (& self , x : u32 , y : u32) -> Self :: Pixel { self . image . get_pixel (x + self . xoffset , y + self . yoffset) } }
};
}
