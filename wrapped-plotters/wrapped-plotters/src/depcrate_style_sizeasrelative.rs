// Generated macro for AsRelative (trait)
macro_rules! Depcrate_style_sizeAsRelative {
() => {
// Module: crate::style::size
// Provides: {"AsRelative"}
// Dependencies: {}
# [doc = " Allows a value turns into a relative size"] pub trait AsRelative : Into < f64 > { # [doc = " Make the value a relative size of percentage of width"] fn percent_width (self) -> RelativeSize { RelativeSize :: Width (self . into () / 100.0) } # [doc = " Make the value a relative size of percentage of height"] fn percent_height (self) -> RelativeSize { RelativeSize :: Height (self . into () / 100.0) } # [doc = " Make the value a relative size of percentage of minimal of height and width"] fn percent (self) -> RelativeSize { RelativeSize :: Smaller (self . into () / 100.0) } }
};
}
