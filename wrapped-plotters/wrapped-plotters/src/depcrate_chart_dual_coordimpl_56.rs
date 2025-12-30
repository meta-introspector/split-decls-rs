// Generated macro for impl_56 (impl)
macro_rules! Depcrate_chart_dual_coordimpl_56 {
() => {
// Module: crate::chart::dual_coord
// Provides: {"impl_56"}
// Dependencies: {}
impl < DB : DrawingBackend , CT1 : CoordTranslate , CT2 : CoordTranslate > DualCoordChartContext < '_ , DB , CT1 , CT2 > { # [doc = " Convert the chart context into a chart state, similar to [ChartContext::into_chart_state](struct.ChartContext.html#method.into_chart_state)"] pub fn into_chart_state (self) -> DualCoordChartState < CT1 , CT2 > { DualCoordChartState { primary : self . primary . into () , secondary : self . secondary . into () , } } # [doc = " Convert the chart context into a sharable chart state."] pub fn into_shared_chart_state (self) -> DualCoordChartState < Arc < CT1 > , Arc < CT2 > > { DualCoordChartState { primary : self . primary . into_shared_chart_state () , secondary : self . secondary . into_shared_chart_state () , } } # [doc = " Copy the coordinate specs and make a chart state"] pub fn to_chart_state (& self) -> DualCoordChartState < CT1 , CT2 > where CT1 : Clone , CT2 : Clone , { DualCoordChartState { primary : self . primary . to_chart_state () , secondary : self . secondary . to_chart_state () , } } }
};
}
