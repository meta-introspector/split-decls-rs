// Generated macro for DottedLineSeries (struct)
macro_rules! Depcrate_series_line_seriesDottedLineSeries {
() => {
// Module: crate::series::line_series
// Provides: {"DottedLineSeries"}
// Dependencies: {}
# [doc = " A dotted line series, map an iterable object to the dotted line element."] pub struct DottedLineSeries < I : Iterator + Clone , Size : SizeDesc , Marker > { points : I , shift : Size , spacing : Size , func : Box < dyn Fn (BackendCoord) -> Marker > , }
};
}
