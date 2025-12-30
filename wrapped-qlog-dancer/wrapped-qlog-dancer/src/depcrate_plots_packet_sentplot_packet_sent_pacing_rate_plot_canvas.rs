// Generated macro for plot_packet_sent_pacing_rate_plot_canvas (function)
macro_rules! Depcrate_plots_packet_sentplot_packet_sent_pacing_rate_plot_canvas {
() => {
// Module: crate::plots::packet_sent
// Provides: {"plot_packet_sent_pacing_rate_plot_canvas"}
// Dependencies: {}
# [cfg (target_arch = "wasm32")] pub fn plot_packet_sent_pacing_rate_plot_canvas < 'a > (params : & PlotParameters , ss : & SeriesStore , ds : & Datastore , canvas_id : & str ,) -> ChartContext < 'a , CanvasBackend , Cartesian2d < RangedCoordf32 , RangedCoordu64 > > { let root = make_chart_canvas_area (canvas_id , params . colors , params . chart_margin) ; draw_pacing_rate_plot (params , ss , ds , & root) }
};
}
