// Generated macro for impl_58 (impl)
macro_rules! Depcrate_chart_dual_coordimpl_58 {
() => {
// Module: crate::chart::dual_coord
// Provides: {"impl_58"}
// Dependencies: {}
impl < DB : DrawingBackend , CT1 : CoordTranslate , CT2 : CoordTranslate > From < DualCoordChartContext < '_ , DB , CT1 , CT2 > > for DualCoordChartState < CT1 , CT2 > { fn from (chart : DualCoordChartContext < '_ , DB , CT1 , CT2 >) -> DualCoordChartState < CT1 , CT2 > { chart . into_chart_state () } }
};
}
