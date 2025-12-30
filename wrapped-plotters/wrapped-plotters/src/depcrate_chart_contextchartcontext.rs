// Generated macro for ChartContext (struct)
macro_rules! Depcrate_chart_contextChartContext {
() => {
// Module: crate::chart::context
// Provides: {"ChartContext"}
// Dependencies: {}
# [doc = "\nThe context of the chart. This is the core object of Plotters.\n\nAny plot/chart is abstracted as this type, and any data series can be placed to the chart context.\n\n- To draw a series on a chart context, use [`ChartContext::draw_series()`].\n- To draw a single element on the chart, you may want to use [`ChartContext::plotting_area()`].\n\nSee [`crate::series::LineSeries`] and [`ChartContext::configure_series_labels()`] for more information and examples\n"] pub struct ChartContext < 'a , DB : DrawingBackend , CT : CoordTranslate > { pub (crate) x_label_area : [Option < DrawingArea < DB , Shift > > ; 2] , pub (crate) y_label_area : [Option < DrawingArea < DB , Shift > > ; 2] , pub (crate) drawing_area : DrawingArea < DB , CT > , pub (crate) series_anno : Vec < SeriesAnno < 'a , DB > > , pub (crate) drawing_area_pos : (i32 , i32) , }
};
}
