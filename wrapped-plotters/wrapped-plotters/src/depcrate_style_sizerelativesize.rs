// Generated macro for RelativeSize (enum)
macro_rules! Depcrate_style_sizeRelativeSize {
() => {
// Module: crate::style::size
// Provides: {"RelativeSize"}
// Dependencies: {}
# [doc = " Describes a relative size, might be"] # [doc = "     1. portion of height"] # [doc = "     2. portion of width"] # [doc = "     3. portion of the minimal of height and weight"] pub enum RelativeSize { # [doc = " Percentage height"] Height (f64) , # [doc = " Percentage width"] Width (f64) , # [doc = " Percentage of either height or width, which is smaller"] Smaller (f64) , }
};
}
