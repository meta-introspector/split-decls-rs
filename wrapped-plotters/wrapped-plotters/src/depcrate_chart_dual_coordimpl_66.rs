// Generated macro for impl_66 (impl)
macro_rules! Depcrate_chart_dual_coordimpl_66 {
() => {
// Module: crate::chart::dual_coord
// Provides: {"impl_66"}
// Dependencies: {}
impl < 'a , DB : DrawingBackend , CT1 : CoordTranslate , CT2 : CoordTranslate > BorrowMut < ChartContext < 'a , DB , CT1 > > for DualCoordChartContext < 'a , DB , CT1 , CT2 > { fn borrow_mut (& mut self) -> & mut ChartContext < 'a , DB , CT1 > { & mut self . primary } }
};
}
