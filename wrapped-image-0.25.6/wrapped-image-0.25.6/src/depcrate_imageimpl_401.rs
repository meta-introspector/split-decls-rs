// Generated macro for impl_401 (impl)
macro_rules! Depcrate_imageimpl_401 {
() => {
// Module: crate::image
// Provides: {"impl_401"}
// Dependencies: {}
# [doc = " Methods for readable images."] impl < I > SubImage < I > where I : Deref , I :: Target : GenericImageView , { # [doc = " Create a sub-view of the image."] # [doc = ""] # [doc = " The coordinates given are relative to the current view on the underlying image."] # [doc = ""] # [doc = " Note that this method is preferred to the one from `GenericImageView`. This is accessible"] # [doc = " with the explicit method call syntax but it should rarely be needed due to causing an"] # [doc = " extra level of indirection."] # [doc = ""] # [doc = " ```"] # [doc = " use image::{GenericImageView, RgbImage, SubImage};"] # [doc = " let buffer = RgbImage::new(10, 10);"] # [doc = ""] # [doc = " let subimage: SubImage<&RgbImage> = buffer.view(0, 0, 10, 10);"] # [doc = " let subview: SubImage<&RgbImage> = subimage.view(0, 0, 10, 10);"] # [doc = ""] # [doc = " // Less efficient and NOT &RgbImage"] # [doc = " let _: SubImage<&_> = GenericImageView::view(&*subimage, 0, 0, 10, 10);"] # [doc = " ```"] pub fn view (& self , x : u32 , y : u32 , width : u32 , height : u32) -> SubImage < & I :: Target > { use crate :: GenericImageView as _ ; assert ! (u64 :: from (x) + u64 :: from (width) <= u64 :: from (self . inner . width ())) ; assert ! (u64 :: from (y) + u64 :: from (height) <= u64 :: from (self . inner . height ())) ; let x = self . inner . xoffset . saturating_add (x) ; let y = self . inner . yoffset . saturating_add (y) ; SubImage :: new (& * self . inner . image , x , y , width , height) } # [doc = " Get a reference to the underlying image."] pub fn inner (& self) -> & I :: Target { & self . inner . image } }
};
}
