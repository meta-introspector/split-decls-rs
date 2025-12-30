// Generated macro for rgb_mul_f32 (function)
macro_rules! Depcrate_rgbrgb_mul_f32 {
() => {
// Module: crate::rgb
// Provides: {"rgb_mul_f32"}
// Dependencies: {}
fn rgb_mul_f32 (lhs : & Rgb , rhs : & f32) -> Rgb { Rgb :: new ((lhs . r as f32 * rhs . clamp (0.0 , 1.0)) as u8 , (lhs . g as f32 * rhs . clamp (0.0 , 1.0)) as u8 , (lhs . b as f32 * rhs . clamp (0.0 , 1.0)) as u8 ,) }
};
}
