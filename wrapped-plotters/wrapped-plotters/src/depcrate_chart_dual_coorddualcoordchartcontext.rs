// Generated macro for DualCoordChartContext (struct)
macro_rules! Depcrate_chart_dual_coordDualCoordChartContext {
() => {
// Module: crate::chart::dual_coord
// Provides: {"DualCoordChartContext"}
// Dependencies: {}
# [doc = " The chart context that has two coordinate system attached."] # [doc = " This situation is quite common, for example, we with two different coordinate system."] # [doc = " For instance this example <img src=\"https://plotters-rs.github.io/plotters-doc-data/twoscale.png\"></img>"] # [doc = " This is done by attaching  a second coordinate system to ChartContext by method [ChartContext::set_secondary_coord](struct.ChartContext.html#method.set_secondary_coord)."] # [doc = " For instance of dual coordinate charts, see [this example](https://github.com/plotters-rs/plotters/blob/master/examples/two-scales.rs#L15)."] # [doc = " Note: `DualCoordChartContext` is always deref to the chart context."] # [doc = " - If you want to configure the secondary axis, method [DualCoordChartContext::configure_secondary_axes](struct.DualCoordChartContext.html#method.configure_secondary_axes)"] # [doc = " - If you want to draw a series using secondary coordinate system, use [DualCoordChartContext::draw_secondary_series](struct.DualCoordChartContext.html#method.draw_secondary_series). And method [ChartContext::draw_series](struct.ChartContext.html#method.draw_series) will always use primary coordinate spec."] pub struct DualCoordChartContext < 'a , DB : DrawingBackend , CT1 : CoordTranslate , CT2 : CoordTranslate > { pub (super) primary : ChartContext < 'a , DB , CT1 > , pub (super) secondary : ChartContext < 'a , DB , CT2 > , }
};
}
