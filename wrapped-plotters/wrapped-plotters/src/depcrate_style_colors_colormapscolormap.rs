// Generated macro for ColorMap (trait)
macro_rules! Depcrate_style_colors_colormapsColorMap {
() => {
// Module: crate::style::colors::colormaps
// Provides: {"ColorMap"}
// Dependencies: {}
# [doc = " Converts scalar values to colors."] pub trait ColorMap < ColorType : crate :: prelude :: Color , FloatType = f32 > where FloatType : num_traits :: Float , { # [doc = " Takes a scalar value 0.0 <= h <= 1.0 and returns the corresponding color."] # [doc = " Typically color-scales are named according to which color-type they return."] # [doc = " To use upper and lower bounds with this function see [get_color_normalized](ColorMap::get_color_normalized)."] fn get_color (& self , h : FloatType) -> ColorType { self . get_color_normalized (h , FloatType :: zero () , FloatType :: one ()) } # [doc = " A slight abstraction over [get_color](ColorMap::get_color) function where lower and upper bound can be specified."] fn get_color_normalized (& self , h : FloatType , min : FloatType , max : FloatType) -> ColorType ; }
};
}
