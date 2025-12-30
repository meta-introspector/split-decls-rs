// Generated macro for adjust_luminance (function)
macro_rules! Depcrate_output_color_scaleadjust_luminance {
() => {
// Module: crate::output::color_scale
// Provides: {"adjust_luminance"}
// Dependencies: {}
fn adjust_luminance (color : Colour , x : f32 , min_l : f32) -> Colour { let rgb_color = match color { Colour :: Rgb (r , g , b) => LinSrgb :: new (f32 :: from (r) / 255.0 , f32 :: from (g) / 255.0 , f32 :: from (b) / 255.0 ,) , Colour :: Black => LinSrgb :: new (0.0 , 0.0 , 0.0) , Colour :: Green | Colour :: LightGreen => LinSrgb :: new (0.0 , 1.0 , 0.0) , Colour :: Yellow | Colour :: LightYellow => LinSrgb :: new (1.0 , 1.0 , 0.0) , Colour :: Blue | Colour :: LightBlue => LinSrgb :: new (0.0 , 0.0 , 1.0) , Colour :: Magenta | Colour :: LightMagenta => LinSrgb :: new (1.0 , 0.0 , 1.0) , Colour :: Cyan | Colour :: LightCyan => LinSrgb :: new (0.0 , 1.0 , 1.0) , Colour :: White => LinSrgb :: new (1.0 , 1.0 , 1.0) , Colour :: LightGray => LinSrgb :: new (0.5 , 0.5 , 0.5) , Colour :: LightRed | Colour :: Red => LinSrgb :: new (1.0 , 0.0 , 0.0) , Colour :: DarkGray => LinSrgb :: new (0.25 , 0.25 , 0.25) , Colour :: LightPurple | Colour :: Purple => LinSrgb :: new (0.5 , 0.0 , 0.5) , _ => LinSrgb :: new (1.0 , 1.0 , 1.0) , } ; let mut lab : Oklab = Oklab :: from_color (rgb_color) ; lab . l = (min_l + (1.0 - min_l) * (- 4.0 * (1.0 - x)) . exp ()) . clamp (0.0 , 1.0) ; let adjusted_rgb : Srgb < f32 > = Srgb :: from_color (lab) ; Colour :: Rgb ((adjusted_rgb . red * 255.0) . round () as u8 , (adjusted_rgb . green * 255.0) . round () as u8 , (adjusted_rgb . blue * 255.0) . round () as u8 ,) }
};
}
