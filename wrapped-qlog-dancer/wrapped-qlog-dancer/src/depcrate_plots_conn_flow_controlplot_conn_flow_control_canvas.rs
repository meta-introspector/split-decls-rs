// Generated macro for plot_conn_flow_control_canvas (function)
macro_rules! Depcrate_plots_conn_flow_controlplot_conn_flow_control_canvas {
() => {
// Module: crate::plots::conn_flow_control
// Provides: {"plot_conn_flow_control_canvas"}
// Dependencies: {}
# [cfg (target_arch = "wasm32")] pub fn plot_conn_flow_control_canvas < 'a > (params : & PlotParameters , filename : & str , ss : & SeriesStore , ds : & Datastore , ty : & ChartOutputType ,) -> ChartContext < 'a , CanvasBackend , Cartesian2d < RangedCoordf32 , RangedCoordu64 > > { let chart_config = make_chart_config ("flow_control" , params , filename , ds , ty) ; let canvas_id : String = chart_config . canvas_id () . unwrap_or_default () ; let root = make_chart_canvas_area (& canvas_id , params . colors , params . chart_margin) ; let axis = XYMinMax :: init (params , ss , ds) ; let mut builder = ChartBuilder :: on (& root) ; builder . x_label_area_size (params . area_margin . x) . y_label_area_size (params . area_margin . y) ; if params . display_chart_title { let caption = format ! ("{} Connection Flow Control timeline" , filename) ; builder . caption (caption , chart_title_style (& params . colors . caption)) ; } let mut fc_chart = builder . build_cartesian_2d (axis . x . range () , axis . y_range ()) . unwrap () ; draw_series (& mut fc_chart , params , ss , ds , axis) ; fc_chart }
};
}
