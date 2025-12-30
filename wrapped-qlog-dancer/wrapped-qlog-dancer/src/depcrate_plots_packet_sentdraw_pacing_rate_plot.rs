// Generated macro for draw_pacing_rate_plot (function)
macro_rules! Depcrate_plots_packet_sentdraw_pacing_rate_plot {
() => {
// Module: crate::plots::packet_sent
// Provides: {"draw_pacing_rate_plot"}
// Dependencies: {}
fn draw_pacing_rate_plot < 'a , DB : DrawingBackend + 'a > (params : & PlotParameters , ss : & SeriesStore , ds : & Datastore , plot : & plotters :: drawing :: DrawingArea < DB , Shift > ,) -> ChartContext < 'a , DB , Cartesian2d < RangedCoordf32 , RangedCoordu64 > > { let y_max = (ss . max_pacing_rate as f32 * Y_WIGGLE) as u64 ; let axis = XYMinMax :: init (params , ss , y_max) ; let mut builder = ChartBuilder :: on (plot) ; builder . x_label_area_size (params . area_margin . x) . y_label_area_size (params . area_margin . y) ; if params . display_chart_title { builder . caption ("Pacing rate" , chart_subtitle_style (& params . colors . caption)) ; } let mut chart = builder . build_cartesian_2d (axis . x . range () , axis . y_range ()) . unwrap () ; draw_mesh (& params . colors , "Relative time (ms)" , "Pacing Rate" , params . display_minor_lines , & mut chart ,) ; chart . draw_series (LineSeries :: new (ds . local_pacing_rate . clone () , RED)) . unwrap () . label ("pacing rate") . legend (| (x , y) | PathElement :: new (vec ! [(x , y) , (x + 20 , y)] , RED)) ; if params . display_legend { chart . configure_series_labels () . label_font (chart_label_style (& params . colors . caption)) . background_style (params . colors . fill . mix (0.8)) . border_style (params . colors . axis) . position (SeriesLabelPosition :: UpperLeft) . draw () . unwrap () ; } chart }
};
}
