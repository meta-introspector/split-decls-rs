// Generated macro for chart_label_style (function)
macro_rules! Depcrate_plotschart_label_style {
() => {
// Module: crate::plots
// Provides: {"chart_label_style"}
// Dependencies: {}
pub fn chart_label_style (color : & RGBColor) -> TextStyle < '_ > { TextStyle :: from (("sans-serif" , 15) . into_font ()) . color (color) }
};
}
