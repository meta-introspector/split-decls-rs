// Generated macro for extended_f (function)
macro_rules! Depcrate_imageops_fast_blurextended_f {
() => {
// Module: crate::imageops::fast_blur
// Provides: {"extended_f"}
// Dependencies: {}
fn extended_f < P : Primitive > (samples : & [P] , width : usize , height : usize , x : isize , y : isize , channel : usize , channel_num : usize ,) -> P { let x = clamp (x , 0 , width as isize - 1) as usize ; let y = clamp (y , 0 , height as isize - 1) as usize ; samples [channel_idx (channel , y * width + x , channel_num)] }
};
}
