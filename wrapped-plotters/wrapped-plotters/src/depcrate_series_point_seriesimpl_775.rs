// Generated macro for impl_775 (impl)
macro_rules! Depcrate_series_point_seriesimpl_775 {
() => {
// Module: crate::series::point_series
// Provides: {"impl_775"}
// Dependencies: {}
impl < 'a , Coord , I : IntoIterator < Item = Coord > , E , Size : SizeDesc + Clone > Iterator for PointSeries < 'a , Coord , I , E , Size > { type Item = E ; fn next (& mut self) -> Option < Self :: Item > { self . data_iter . next () . map (| x | (self . make_point) (x , self . size . clone () , self . style)) } }
};
}
