// Generated macro for OverviewChartOutputType (enum)
macro_rules! Depcrate_plots_conn_overviewOverviewChartOutputType {
() => {
// Module: crate::plots::conn_overview
// Provides: {"OverviewChartOutputType"}
// Dependencies: {}
# [derive (Clone , Debug)] pub enum OverviewChartOutputType { Png { output_dir : String , cwnd_y_max : Option < u64 > , stream_y_max : Option < u64 > , } , Canvas { main_plot_canvas_id : String , congestion_plot_canvas_id : String , rtt_plot_canvas_id : String , } , }
};
}
