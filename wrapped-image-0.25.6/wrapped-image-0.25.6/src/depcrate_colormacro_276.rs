// Generated macro for macro_276 (macro)
macro_rules! Depcrate_colormacro_276 {
() => {
// Module: crate::color
// Provides: {"macro_276"}
// Dependencies: {}
define_colors ! { # [doc = " RGB colors."] # [doc = ""] # [doc = " For the purpose of color conversion, as well as blending, the implementation of `Pixel`"] # [doc = " assumes an `sRGB` color space of its data."] pub struct Rgb < T : Primitive Enlargeable > ([T ; 3 , 0]) = "RGB" ; # [doc = " Grayscale colors."] pub struct Luma < T : Primitive > ([T ; 1 , 0]) = "Y" ; # [doc = " RGB colors + alpha channel"] pub struct Rgba < T : Primitive Enlargeable > ([T ; 4 , 1]) = "RGBA" ; # [doc = " Grayscale colors + alpha channel"] pub struct LumaA < T : Primitive > ([T ; 2 , 1]) = "YA" ; }
};
}
