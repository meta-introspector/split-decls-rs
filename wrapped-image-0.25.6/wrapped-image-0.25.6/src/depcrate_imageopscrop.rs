// Generated macro for crop (function)
macro_rules! Depcrate_imageopscrop {
() => {
// Module: crate::imageops
// Provides: {"crop"}
// Dependencies: {}
# [doc = " Return a mutable view into an image"] # [doc = " The coordinates set the position of the top left corner of the crop."] pub fn crop < I : GenericImageView > (image : & mut I , x : u32 , y : u32 , width : u32 , height : u32 ,) -> SubImage < & mut I > { let (x , y , width , height) = crop_dimms (image , x , y , width , height) ; SubImage :: new (image , x , y , width , height) }
};
}
