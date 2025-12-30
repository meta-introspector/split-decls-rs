// Generated macro for impl_59 (impl)
macro_rules! Depcrate_chart_dual_coordimpl_59 {
() => {
// Module: crate::chart::dual_coord
// Provides: {"impl_59"}
// Dependencies: {}
impl < 'b , DB : DrawingBackend , CT1 : CoordTranslate + Clone , CT2 : CoordTranslate + Clone > From < & 'b DualCoordChartContext < '_ , DB , CT1 , CT2 > > for DualCoordChartState < CT1 , CT2 > { fn from (chart : & 'b DualCoordChartContext < '_ , DB , CT1 , CT2 >) -> DualCoordChartState < CT1 , CT2 > { chart . to_chart_state () } }
};
}
