// Generated macro for chart_title_style (function)
macro_rules! Depcrate_plotschart_title_style {
() => {
// Module: crate::plots
// Provides: {"chart_title_style"}
// Dependencies: {}
pub fn chart_title_style (color : & RGBColor) -> TextStyle < '_ > { TextStyle :: from (("sans-serif" , 40) . into_font ()) . color (color) }
};
}
