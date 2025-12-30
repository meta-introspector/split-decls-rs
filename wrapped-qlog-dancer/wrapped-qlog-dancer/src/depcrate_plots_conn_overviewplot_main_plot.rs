// Generated macro for plot_main_plot (function)
macro_rules! Depcrate_plots_conn_overviewplot_main_plot {
() => {
// Module: crate::plots::conn_overview
// Provides: {"plot_main_plot"}
// Dependencies: {}
# [cfg (target_arch = "wasm32")] pub fn plot_main_plot < 'a > (params : & PlotParameters , filename : & str , ss : & SeriesStore , canvas_id : & str ,) -> ChartContext < 'a , CanvasBackend , Cartesian2d < RangedCoordf32 , RangedCoordu64 > > { let root = make_chart_canvas_area (& canvas_id , params . colors , params . chart_margin) ; let stream_y_max = if let Some (y_max) = params . clamp . stream_y_max { y_max } else { ss . y_max_stream_plot } ; let stream_axis = XYMinMax :: init (params , ss , stream_y_max) ; draw_main_plot (filename , params , stream_axis , ss , & root) }
};
}
