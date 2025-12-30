// Generated macro for PointSeries (struct)
macro_rules! Depcrate_series_point_seriesPointSeries {
() => {
// Module: crate::series::point_series
// Provides: {"PointSeries"}
// Dependencies: {}
# [doc = " The point plot object, which takes an iterator of points in guest coordinate system"] # [doc = " and create an element for each point"] pub struct PointSeries < 'a , Coord , I : IntoIterator < Item = Coord > , E , Size : SizeDesc + Clone > { style : ShapeStyle , size : Size , data_iter : I :: IntoIter , make_point : & 'a dyn Fn (Coord , Size , ShapeStyle) -> E , }
};
}
