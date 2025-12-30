// Generated macro for DerivedColorMap (struct)
macro_rules! Depcrate_style_colors_colormapsDerivedColorMap {
() => {
// Module: crate::style::colors::colormaps
// Provides: {"DerivedColorMap"}
// Dependencies: {}
# [doc = " This struct is used to dynamically construct colormaps by giving it a slice of colors."] # [doc = " It can then be used when being intantiated, but not with associated functions."] # [doc = " ```"] # [doc = " use plotters::prelude::{BLACK,BLUE,WHITE,DerivedColorMap,ColorMap};"] # [doc = ""] # [doc = " let derived_colormap = DerivedColorMap::new("] # [doc = "     &[BLACK,"] # [doc = "     BLUE,"] # [doc = "     WHITE]"] # [doc = " );"] # [doc = ""] # [doc = " assert_eq!(derived_colormap.get_color(0.0), BLACK);"] # [doc = " assert_eq!(derived_colormap.get_color(0.5), BLUE);"] # [doc = " assert_eq!(derived_colormap.get_color(1.0), WHITE);"] # [doc = " ```"] pub struct DerivedColorMap < ColorType > { colors : Vec < ColorType > , }
};
}
