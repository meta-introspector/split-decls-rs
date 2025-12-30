// Generated macro for impl_65 (impl)
macro_rules! Depcrate_chart_dual_coordimpl_65 {
() => {
// Module: crate::chart::dual_coord
// Provides: {"impl_65"}
// Dependencies: {}
impl < 'a , DB : DrawingBackend , CT1 : CoordTranslate , CT2 : CoordTranslate > Borrow < ChartContext < 'a , DB , CT1 > > for DualCoordChartContext < 'a , DB , CT1 , CT2 > { fn borrow (& self) -> & ChartContext < 'a , DB , CT1 > { & self . primary } }
};
}
