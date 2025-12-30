// Generated macro for impl_777 (impl)
macro_rules! Depcrate_series_point_seriesimpl_777 {
() => {
// Module: crate::series::point_series
// Provides: {"impl_777"}
// Dependencies: {}
impl < 'a , Coord , I : IntoIterator < Item = Coord > , E , Size : SizeDesc + Clone > PointSeries < 'a , Coord , I , E , Size > { # [doc = " Create a new point series. Similar to `PointSeries::new` but it doesn't"] # [doc = " requires the element implements point trait. So instead of using the point"] # [doc = " constructor, it uses the customized function for element creation"] pub fn of_element < S : Into < ShapeStyle > , F : Fn (Coord , Size , ShapeStyle) -> E > (iter : I , size : Size , style : S , cons : & 'a F ,) -> Self { Self { data_iter : iter . into_iter () , size , style : style . into () , make_point : cons , } } }
};
}
