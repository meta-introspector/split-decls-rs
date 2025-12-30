// Generated macro for impl_226 (impl)
macro_rules! Depcrate_animationimpl_226 {
() => {
// Module: crate::animation
// Provides: {"impl_226"}
// Dependencies: {}
impl Frame { # [doc = " Constructs a new frame without any delay."] # [must_use] pub fn new (buffer : RgbaImage) -> Frame { Frame { delay : Delay :: from_ratio (Ratio { numer : 0 , denom : 1 }) , left : 0 , top : 0 , buffer , } } # [doc = " Constructs a new frame"] # [must_use] pub fn from_parts (buffer : RgbaImage , left : u32 , top : u32 , delay : Delay) -> Frame { Frame { delay , left , top , buffer , } } # [doc = " Delay of this frame"] # [must_use] pub fn delay (& self) -> Delay { self . delay } # [doc = " Returns the image buffer"] # [must_use] pub fn buffer (& self) -> & RgbaImage { & self . buffer } # [doc = " Returns a mutable image buffer"] pub fn buffer_mut (& mut self) -> & mut RgbaImage { & mut self . buffer } # [doc = " Returns the image buffer"] # [must_use] pub fn into_buffer (self) -> RgbaImage { self . buffer } # [doc = " Returns the x offset"] # [must_use] pub fn left (& self) -> u32 { self . left } # [doc = " Returns the y offset"] # [must_use] pub fn top (& self) -> u32 { self . top } }
};
}
