// Generated macro for impl_768 (impl)
macro_rules! Depcrate_series_line_seriesimpl_768 {
() => {
// Module: crate::series::line_series
// Provides: {"impl_768"}
// Dependencies: {}
impl < I : Iterator + Clone , Size : SizeDesc , Marker > DottedLineSeries < I , Size , Marker > { # [doc = " Create a new line series from"] # [doc = " - `points`: The iterator of the points"] # [doc = " - `shift`: The shift of the first marker"] # [doc = " - `spacing`: The spacing between markers"] # [doc = " - `func`: The marker function"] # [doc = " - returns the created element"] pub fn new < I0 , F > (points : I0 , shift : Size , spacing : Size , func : F) -> Self where I0 : IntoIterator < IntoIter = I > , F : Fn (BackendCoord) -> Marker + 'static , { Self { points : points . into_iter () , shift , spacing , func : Box :: new (func) , } } }
};
}
