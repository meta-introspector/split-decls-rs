// Generated macro for impl_733 (impl)
macro_rules! Depcrate_series_area_seriesimpl_733 {
() => {
// Module: crate::series::area_series
// Provides: {"impl_733"}
// Dependencies: {}
impl < DB : DrawingBackend , X : Clone , Y : Clone > AreaSeries < DB , X , Y > { # [doc = "\n    Creates an area series with transparent border.\n\n    See [`AreaSeries`] for more information and examples.\n    "] pub fn new < S : Into < ShapeStyle > , I : IntoIterator < Item = (X , Y) > > (iter : I , baseline : Y , area_style : S ,) -> Self { Self { area_style : area_style . into () , baseline , data : iter . into_iter () . collect () , state : 0 , border_style : (& TRANSPARENT) . into () , _p : std :: marker :: PhantomData , } } # [doc = "\n    Sets the border style of the area series.\n\n    See [`AreaSeries`] for more information and examples.\n    "] pub fn border_style < S : Into < ShapeStyle > > (mut self , style : S) -> Self { self . border_style = style . into () ; self } }
};
}
