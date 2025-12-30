// Generated macro for impl_839 (impl)
macro_rules! Depcrate_style_colors_colormapsimpl_839 {
() => {
// Module: crate::style::colors::colormaps
// Provides: {"impl_839"}
// Dependencies: {}
impl < ColorType : crate :: style :: Color + Clone > DerivedColorMap < ColorType > { # [doc = " This function lets the user define a new colormap by simply specifying colors in the correct order."] # [doc = " For calculation of the color values, the colors will be spaced evenly apart."] pub fn new (colors : & [ColorType]) -> Self { DerivedColorMap { colors : colors . to_vec () , } } }
};
}
