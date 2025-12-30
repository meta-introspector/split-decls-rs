// Generated macro for crop_dimms (function)
macro_rules! Depcrate_imageopscrop_dimms {
() => {
// Module: crate::imageops
// Provides: {"crop_dimms"}
// Dependencies: {}
fn crop_dimms < I : GenericImageView > (image : & I , x : u32 , y : u32 , width : u32 , height : u32 ,) -> (u32 , u32 , u32 , u32) { let (iwidth , iheight) = image . dimensions () ; let x = cmp :: min (x , iwidth) ; let y = cmp :: min (y , iheight) ; let height = cmp :: min (height , iheight - y) ; let width = cmp :: min (width , iwidth - x) ; (x , y , width , height) }
};
}
