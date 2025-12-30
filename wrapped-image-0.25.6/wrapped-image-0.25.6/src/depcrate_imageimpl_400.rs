// Generated macro for impl_400 (impl)
macro_rules! Depcrate_imageimpl_400 {
() => {
// Module: crate::image
// Provides: {"impl_400"}
// Dependencies: {}
impl < I > SubImage < I > { # [doc = " Construct a new subimage"] # [doc = " The coordinates set the position of the top left corner of the `SubImage`."] pub fn new (image : I , x : u32 , y : u32 , width : u32 , height : u32) -> SubImage < I > { SubImage { inner : SubImageInner { image , xoffset : x , yoffset : y , xstride : width , ystride : height , } , } } # [doc = " Change the coordinates of this subimage."] pub fn change_bounds (& mut self , x : u32 , y : u32 , width : u32 , height : u32) { self . inner . xoffset = x ; self . inner . yoffset = y ; self . inner . xstride = width ; self . inner . ystride = height ; } # [doc = " The offsets of this subimage relative to the underlying image."] pub fn offsets (& self) -> (u32 , u32) { (self . inner . xoffset , self . inner . yoffset) } # [doc = " Convert this subimage to an `ImageBuffer`"] pub fn to_image (& self) -> ImageBuffer < DerefPixel < I > , Vec < DerefSubpixel < I > > > where I : Deref , I :: Target : GenericImageView + 'static , { let mut out = ImageBuffer :: new (self . inner . xstride , self . inner . ystride) ; let borrowed = & * self . inner . image ; for y in 0 .. self . inner . ystride { for x in 0 .. self . inner . xstride { let p = borrowed . get_pixel (x + self . inner . xoffset , y + self . inner . yoffset) ; out . put_pixel (x , y , p) ; } } out } }
};
}
