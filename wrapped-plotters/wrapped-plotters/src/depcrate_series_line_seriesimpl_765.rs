// Generated macro for impl_765 (impl)
macro_rules! Depcrate_series_line_seriesimpl_765 {
() => {
// Module: crate::series::line_series
// Provides: {"impl_765"}
// Dependencies: {}
impl < I : Iterator + Clone , Size : SizeDesc > DashedLineSeries < I , Size > { # [doc = " Create a new line series from"] # [doc = " - `points`: The iterator of the points"] # [doc = " - `size`: The dash size"] # [doc = " - `spacing`: The dash-to-dash spacing (gap size)"] # [doc = " - `style`: The shape style"] # [doc = " - returns the created element"] pub fn new < I0 > (points : I0 , size : Size , spacing : Size , style : ShapeStyle) -> Self where I0 : IntoIterator < IntoIter = I > , { Self { points : points . into_iter () , size , spacing , style , } } }
};
}
