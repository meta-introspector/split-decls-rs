// Generated macro for impl_402 (impl)
macro_rules! Depcrate_imageimpl_402 {
() => {
// Module: crate::image
// Provides: {"impl_402"}
// Dependencies: {}
impl < I > SubImage < I > where I : DerefMut , I :: Target : GenericImage , { # [doc = " Create a mutable sub-view of the image."] # [doc = ""] # [doc = " The coordinates given are relative to the current view on the underlying image."] pub fn sub_image (& mut self , x : u32 , y : u32 , width : u32 , height : u32 ,) -> SubImage < & mut I :: Target > { assert ! (u64 :: from (x) + u64 :: from (width) <= u64 :: from (self . inner . width ())) ; assert ! (u64 :: from (y) + u64 :: from (height) <= u64 :: from (self . inner . height ())) ; let x = self . inner . xoffset . saturating_add (x) ; let y = self . inner . yoffset . saturating_add (y) ; SubImage :: new (& mut * self . inner . image , x , y , width , height) } # [doc = " Get a mutable reference to the underlying image."] pub fn inner_mut (& mut self) -> & mut I :: Target { & mut self . inner . image } }
};
}
