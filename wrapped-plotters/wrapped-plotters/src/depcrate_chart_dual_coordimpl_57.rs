// Generated macro for impl_57 (impl)
macro_rules! Depcrate_chart_dual_coordimpl_57 {
() => {
// Module: crate::chart::dual_coord
// Provides: {"impl_57"}
// Dependencies: {}
impl < CT1 : CoordTranslate , CT2 : CoordTranslate > DualCoordChartState < CT1 , CT2 > { # [doc = " Restore the chart state on the given drawing area"] pub fn restore < DB : DrawingBackend > (self , area : & DrawingArea < DB , Shift > ,) -> DualCoordChartContext < '_ , DB , CT1 , CT2 > { let primary = self . primary . restore (area) ; let secondary = self . secondary . restore (& primary . plotting_area () . strip_coord_spec ()) ; DualCoordChartContext { primary , secondary } } }
};
}
