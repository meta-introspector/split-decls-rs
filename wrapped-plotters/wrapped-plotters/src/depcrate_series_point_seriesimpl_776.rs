// Generated macro for impl_776 (impl)
macro_rules! Depcrate_series_point_seriesimpl_776 {
() => {
// Module: crate::series::point_series
// Provides: {"impl_776"}
// Dependencies: {}
impl < 'a , Coord , I : IntoIterator < Item = Coord > , E , Size : SizeDesc + Clone > PointSeries < 'a , Coord , I , E , Size > where E : PointElement < Coord , Size > , { # [doc = " Create a new point series with the element that implements point trait."] # [doc = " You may also use a more general way to create a point series with `of_element`"] # [doc = " function which allows a customized element construction function"] pub fn new < S : Into < ShapeStyle > > (iter : I , size : Size , style : S) -> Self { Self { data_iter : iter . into_iter () , size , style : style . into () , make_point : & | a , b , c | E :: make_point (a , b , c) , } } }
};
}
