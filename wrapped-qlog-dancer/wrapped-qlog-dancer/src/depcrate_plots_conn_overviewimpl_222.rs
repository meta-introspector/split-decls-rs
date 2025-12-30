// Generated macro for impl_222 (impl)
macro_rules! Depcrate_plots_conn_overviewimpl_222 {
() => {
// Module: crate::plots::conn_overview
// Provides: {"impl_222"}
// Dependencies: {}
impl From < OverviewChartOutputType > for ChartOutputType { fn from (val : OverviewChartOutputType) -> Self { match val { OverviewChartOutputType :: Png { output_dir , cwnd_y_max , stream_y_max , } => ChartOutputType :: Png { output_dir , cwnd_y_max , stream_y_max , } , OverviewChartOutputType :: Canvas { main_plot_canvas_id , .. } => ChartOutputType :: Canvas { canvas_id : main_plot_canvas_id , } , } } }
};
}
