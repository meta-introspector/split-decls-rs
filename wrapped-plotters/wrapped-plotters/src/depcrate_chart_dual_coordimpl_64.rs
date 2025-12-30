// Generated macro for impl_64 (impl)
macro_rules! Depcrate_chart_dual_coordimpl_64 {
() => {
// Module: crate::chart::dual_coord
// Provides: {"impl_64"}
// Dependencies: {}
impl < 'a , DB : DrawingBackend , X : Ranged , Y : Ranged , SX : Ranged , SY : Ranged > DualCoordChartContext < 'a , DB , Cartesian2d < X , Y > , Cartesian2d < SX , SY > > { # [doc = " Draw a series use the secondary coordinate system."] # [doc = " - `series`: The series to draw"] # [doc = " - `Returns` the series annotation object or error code"] pub fn draw_secondary_series < E , R , S > (& mut self , series : S ,) -> Result < & mut SeriesAnno < 'a , DB > , DrawingAreaErrorKind < DB :: ErrorType > > where for < 'b > & 'b E : PointCollection < 'b , (SX :: ValueType , SY :: ValueType) > , E : Drawable < DB > , R : Borrow < E > , S : IntoIterator < Item = R > , { self . secondary . draw_series_impl (series) ? ; Ok (self . primary . alloc_series_anno ()) } }
};
}
