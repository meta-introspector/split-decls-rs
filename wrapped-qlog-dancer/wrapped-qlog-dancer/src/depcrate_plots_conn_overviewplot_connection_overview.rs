// Generated macro for plot_connection_overview (function)
macro_rules! Depcrate_plots_conn_overviewplot_connection_overview {
() => {
// Module: crate::plots::conn_overview
// Provides: {"plot_connection_overview"}
// Dependencies: {}
# [cfg (not (target_arch = "wasm32"))] pub fn plot_connection_overview (params : & PlotParameters , filename : & str , ss : & SeriesStore , ds : & Datastore , ty : & OverviewChartOutputType ,) { let chart_config = ChartConfig { title : "conn-overview" . into () , input_filename : filename . into () , clamp : params . clamp . clone () , app_proto : ds . application_proto , host : ds . host . clone () , session_id : ds . session_id , ty : ty . clone () . into () , } ; chart_config . init_chart_dir () ; let chart_path = chart_config . chart_filepath () ; let root = make_chart_bitmap_area (& chart_path , params . chart_size , params . colors , params . chart_margin ,) ; let (stream_plot , remainder) = root . split_vertically ((60) . percent ()) ; let (congestion_plot , rtt_plot) = remainder . split_vertically ((60) . percent ()) ; let stream_y_max = if let Some (y_max) = params . clamp . stream_y_max { y_max } else { ss . y_max_stream_plot } ; let stream_axis = XYMinMax :: init (params , ss , stream_y_max) ; let cwnd_y_max = if let Some (y_max) = params . cwnd_y_max { y_max } else { ss . y_max_congestion_plot + ss . y_max_congestion_plot / 10 } ; let common_axis = super :: minmax :: XYMinMax :: init (ss . sent_x_min .. ss . sent_x_max , params . clamp . start , params . clamp . end , 0 .. cwnd_y_max ,) ; draw_main_plot (filename , params , stream_axis , ss , & stream_plot) ; draw_congestion_plot (params , & common_axis , ss , ds , & congestion_plot) ; draw_rtt_plot (params , & common_axis , ss , & rtt_plot) ; }
};
}
