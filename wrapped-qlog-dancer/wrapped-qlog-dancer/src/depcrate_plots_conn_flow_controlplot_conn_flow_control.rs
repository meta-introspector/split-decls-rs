// Generated macro for plot_conn_flow_control (function)
macro_rules! Depcrate_plots_conn_flow_controlplot_conn_flow_control {
() => {
// Module: crate::plots::conn_flow_control
// Provides: {"plot_conn_flow_control"}
// Dependencies: {}
# [cfg (not (target_arch = "wasm32"))] pub fn plot_conn_flow_control (params : & PlotParameters , filename : & str , ss : & SeriesStore , ds : & Datastore , ty : & ChartOutputType ,) { let chart_config = make_chart_config ("flow_control" , params , filename , ds , ty) ; let chart_path = chart_config . chart_filepath () ; let root = make_chart_bitmap_area (& chart_path , params . chart_size , params . colors , params . chart_margin ,) ; let axis = XYMinMax :: init (params , ss , ds) ; let mut builder = ChartBuilder :: on (& root) ; builder . x_label_area_size (params . area_margin . x) . y_label_area_size (params . area_margin . y) ; if params . display_chart_title { let caption = format ! ("{} Connection Flow Control timeline" , filename) ; builder . caption (caption , chart_title_style (& params . colors . caption)) ; } let mut fc_chart = builder . build_cartesian_2d (axis . x . range () , axis . y_range ()) . unwrap () ; draw_mesh (& params . colors , "Relative time (ms)" , "Bytes" , params . display_minor_lines , & mut fc_chart ,) ; draw_series (& mut fc_chart , params , ss , ds , axis) ; }
};
}
