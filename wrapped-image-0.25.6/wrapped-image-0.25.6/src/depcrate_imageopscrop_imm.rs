// Generated macro for crop_imm (function)
macro_rules! Depcrate_imageopscrop_imm {
() => {
// Module: crate::imageops
// Provides: {"crop_imm"}
// Dependencies: {}
# [doc = " Return an immutable view into an image"] # [doc = " The coordinates set the position of the top left corner of the crop."] pub fn crop_imm < I : GenericImageView > (image : & I , x : u32 , y : u32 , width : u32 , height : u32 ,) -> SubImage < & I > { let (x , y , width , height) = crop_dimms (image , x , y , width , height) ; SubImage :: new (image , x , y , width , height) }
};
}
