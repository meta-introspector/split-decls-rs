// Generated macro for draw_rtt_plot (function)
macro_rules! Depcrate_plots_rttdraw_rtt_plot {
() => {
// Module: crate::plots::rtt
// Provides: {"draw_rtt_plot"}
// Dependencies: {}
pub fn draw_rtt_plot < 'a , DB : DrawingBackend + 'a > (params : & PlotParameters , axis : & XYMinMax < u64 > , ss : & SeriesStore , plot : & plotters :: drawing :: DrawingArea < DB , Shift > ,) -> ChartContext < 'a , DB , Cartesian2d < RangedCoordf32 , RangedCoordf32 > > { let mut builder = ChartBuilder :: on (plot) ; builder . x_label_area_size (params . area_margin . x) . y_label_area_size (params . area_margin . y) ; if params . display_chart_title { builder . caption ("RTT" , chart_subtitle_style (& params . colors . caption)) ; } let mut chart = builder . build_cartesian_2d (axis . x . range () , 0.0 .. (ss . y_max_rtt_plot + ss . y_max_rtt_plot / 10.0) ,) . unwrap () ; draw_mesh (& params . colors , "Relative time (ms)" , "RTT (ms)" , false , & mut chart ,) ; draw_min_rtt (& ss . local_min_rtt , & mut chart) ; draw_latest_rtt (& ss . local_latest_rtt , & mut chart) ; draw_smoothed_rtt (& ss . local_smoothed_rtt , & mut chart) ; if params . display_legend { chart . configure_series_labels () . label_font (chart_label_style (& params . colors . caption)) . background_style (params . colors . fill . mix (0.8)) . border_style (params . colors . axis) . position (SeriesLabelPosition :: UpperLeft) . draw () . unwrap () ; } chart }
};
}
