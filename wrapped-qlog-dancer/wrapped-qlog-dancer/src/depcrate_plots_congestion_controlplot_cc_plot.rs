// Generated macro for plot_cc_plot (function)
macro_rules! Depcrate_plots_congestion_controlplot_cc_plot {
() => {
// Module: crate::plots::congestion_control
// Provides: {"plot_cc_plot"}
// Dependencies: {}
# [cfg (target_arch = "wasm32")] pub fn plot_cc_plot < 'a > (params : & PlotParameters , ss : & SeriesStore , ds : & Datastore , canvas_id : & str ,) -> ChartContext < 'a , CanvasBackend , Cartesian2d < RangedCoordf32 , RangedCoordu64 > > { let root = make_chart_canvas_area (& canvas_id , params . colors , params . chart_margin) ; let cwnd_y_max = if let Some (y_max) = params . cwnd_y_max { y_max } else { ss . y_max_congestion_plot + ss . y_max_congestion_plot / 10 } ; let axis = super :: minmax :: XYMinMax :: init (ss . sent_x_min .. ss . sent_x_max , params . clamp . start , params . clamp . end , 0 .. cwnd_y_max ,) ; draw_congestion_plot (params , & axis , ss , ds , & root) }
};
}
