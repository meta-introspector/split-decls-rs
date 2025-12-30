// Generated macro for impl_67 (impl)
macro_rules! Depcrate_chart_dual_coordimpl_67 {
() => {
// Module: crate::chart::dual_coord
// Provides: {"impl_67"}
// Dependencies: {}
impl < 'a , DB : DrawingBackend , CT1 : CoordTranslate , CT2 : CoordTranslate > Deref for DualCoordChartContext < 'a , DB , CT1 , CT2 > { type Target = ChartContext < 'a , DB , CT1 > ; fn deref (& self) -> & Self :: Target { self . borrow () } }
};
}
