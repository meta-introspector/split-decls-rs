// Generated macro for DualCoordChartState (struct)
macro_rules! Depcrate_chart_dual_coordDualCoordChartState {
() => {
// Module: crate::chart::dual_coord
// Provides: {"DualCoordChartState"}
// Dependencies: {}
# [doc = " The chart state for a dual coord chart, see the detailed description for `ChartState` for more"] # [doc = " information about the purpose of a chart state."] # [doc = " Similar to [ChartState](struct.ChartState.html), but used for the dual coordinate charts."] # [derive (Clone)] pub struct DualCoordChartState < CT1 : CoordTranslate , CT2 : CoordTranslate > { primary : ChartState < CT1 > , secondary : ChartState < CT2 > , }
};
}
