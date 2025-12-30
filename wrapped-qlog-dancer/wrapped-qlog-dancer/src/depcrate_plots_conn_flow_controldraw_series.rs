// Generated macro for draw_series (function)
macro_rules! Depcrate_plots_conn_flow_controldraw_series {
() => {
// Module: crate::plots::conn_flow_control
// Provides: {"draw_series"}
// Dependencies: {}
fn draw_series < 'a , DB : DrawingBackend + 'a > (chart : & mut ChartContext < 'a , DB , Cartesian2d < RangedCoordf32 , RangedCoordu64 > > , params : & PlotParameters , ss : & SeriesStore , ds : & Datastore , axis : XYMinMax ,) { draw_mesh (& params . colors , "Relative time (ms)" , "Bytes" , params . display_minor_lines , chart ,) ; blocked_lines (ds , axis . y_max , chart) ; received_data (ss , & ds . application_proto , chart) ; window_updates (ss , ds , chart) ; if params . display_legend { chart . configure_series_labels () . label_font (chart_label_style (& params . colors . caption)) . background_style (params . colors . fill . mix (0.8)) . border_style (params . colors . axis) . position (SeriesLabelPosition :: LowerRight) . draw () . unwrap () ; } }
};
}
