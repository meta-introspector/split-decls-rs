// Generated macro for impl_61 (impl)
macro_rules! Depcrate_chart_dual_coordimpl_61 {
() => {
// Module: crate::chart::dual_coord
// Provides: {"impl_61"}
// Dependencies: {}
impl < DB : DrawingBackend , CT1 : CoordTranslate , CT2 : ReverseCoordTranslate > DualCoordChartContext < '_ , DB , CT1 , CT2 > { # [doc = " Convert the chart context into the secondary coordinate translation function"] pub fn into_secondary_coord_trans (self) -> impl Fn (BackendCoord) -> Option < CT2 :: From > { let coord_spec = self . secondary . drawing_area . into_coord_spec () ; move | coord | coord_spec . reverse_translate (coord) } }
};
}
