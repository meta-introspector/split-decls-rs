// Generated macro for plot_packet_sent_delta_plot_canvas (function)
macro_rules! Depcrate_plots_packet_sentplot_packet_sent_delta_plot_canvas {
() => {
// Module: crate::plots::packet_sent
// Provides: {"plot_packet_sent_delta_plot_canvas"}
// Dependencies: {}
# [cfg (target_arch = "wasm32")] pub fn plot_packet_sent_delta_plot_canvas < 'a > (params : & PlotParameters , ss : & SeriesStore , canvas_id : & str ,) -> ChartContext < 'a , CanvasBackend , Cartesian2d < RangedCoordu64 , RangedCoordf32 > > { let root = make_chart_canvas_area (canvas_id , params . colors , params . chart_margin) ; draw_delta_plot (params , ss , & root) }
};
}
