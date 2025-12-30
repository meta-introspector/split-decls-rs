// Generated macro for impl_763 (impl)
macro_rules! Depcrate_series_line_seriesimpl_763 {
() => {
// Module: crate::series::line_series
// Provides: {"impl_763"}
// Dependencies: {}
impl < DB : DrawingBackend , Coord > LineSeries < DB , Coord > { # [doc = "\n    Creates a new line series based on a data iterator and a given style.\n\n    See [`LineSeries`] for more information and examples.\n    "] pub fn new < I : IntoIterator < Item = Coord > , S : Into < ShapeStyle > > (iter : I , style : S) -> Self { Self { style : style . into () , data : iter . into_iter () . collect () , point_size : 0 , point_idx : 0 , phantom : PhantomData , } } # [doc = "\n    Sets the size of the points in the series, in pixels.\n\n    See [`LineSeries`] for more information and examples.\n    "] pub fn point_size (mut self , size : u32) -> Self { self . point_size = size ; self } }
};
}
