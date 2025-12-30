// Generated macro for draw_congestion_plot (function)
macro_rules! Depcrate_plots_congestion_controldraw_congestion_plot {
() => {
// Module: crate::plots::congestion_control
// Provides: {"draw_congestion_plot"}
// Dependencies: {}
pub fn draw_congestion_plot < 'a , DB : DrawingBackend + 'a > (params : & PlotParameters , axis : & XYMinMax < u64 > , ss : & SeriesStore , ds : & Datastore , plot : & plotters :: drawing :: DrawingArea < DB , Shift > ,) -> ChartContext < 'a , DB , Cartesian2d < RangedCoordf32 , RangedCoordu64 > > { let mut builder = ChartBuilder :: on (plot) ; builder . x_label_area_size (params . area_margin . x) . y_label_area_size (params . area_margin . y) ; if params . display_chart_title { builder . caption ("Congestion" , chart_subtitle_style (& params . colors . caption)) ; } let extended_y = axis . y_range . start .. axis . y_range . end + (axis . y_range . end / 10) * 5 ; let mut plot = builder . build_cartesian_2d (axis . x . range () , extended_y . clone ()) . unwrap () ; draw_mesh (& params . colors , "Relative time (ms)" , "Data (bytes)" , params . display_minor_lines , & mut plot ,) ; draw_cc_updates (& ds . congestion_state_updates , axis . y_range . clone () , extended_y , & mut plot ,) ; draw_bytes_in_flight (& ss . local_bytes_in_flight , & mut plot) ; draw_cwnd (& ss . local_cwnd , & mut plot) ; draw_ssthresh (& ss . local_ssthresh , & mut plot) ; if params . display_legend { plot . configure_series_labels () . label_font (chart_label_style (& params . colors . caption)) . background_style (params . colors . fill . mix (0.8)) . border_style (params . colors . axis) . position (SeriesLabelPosition :: UpperLeft) . draw () . unwrap () ; } plot }
};
}
