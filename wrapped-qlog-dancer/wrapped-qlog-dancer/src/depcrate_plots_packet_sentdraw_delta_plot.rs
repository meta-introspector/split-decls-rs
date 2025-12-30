// Generated macro for draw_delta_plot (function)
macro_rules! Depcrate_plots_packet_sentdraw_delta_plot {
() => {
// Module: crate::plots::packet_sent
// Provides: {"draw_delta_plot"}
// Dependencies: {}
fn draw_delta_plot < 'a , DB : DrawingBackend + 'a > (params : & PlotParameters , ss : & SeriesStore , plot : & plotters :: drawing :: DrawingArea < DB , Shift > ,) -> ChartContext < 'a , DB , Cartesian2d < RangedCoordu64 , RangedCoordf32 > > { let y_range = ss . y_min_onertt_packet_created_sent_delta .. (ss . y_max_onertt_packet_created_sent_delta * Y_WIGGLE) ; let mut builder = ChartBuilder :: on (plot) ; builder . x_label_area_size (params . area_margin . x) . y_label_area_size (params . area_margin . y) ; if params . display_chart_title { builder . caption ("Packet created/sent Delta timing" , chart_subtitle_style (& params . colors . caption) ,) ; } let mut chart = builder . build_cartesian_2d (0 .. ss . y_max_onertt_pkt_sent_plot , y_range) . unwrap () ; draw_mesh (& params . colors , "Packet Number" , "Delta time (ms)" , params . display_minor_lines , & mut chart ,) ; let lines = LineSeries :: new (ss . onertt_packet_created_sent_delta . clone () , BLUE) ; let _crosses = ss . onertt_packet_created_sent_delta . iter () . map (| point | Cross :: new (* point , 2 , BLUE)) ; chart . draw_series (lines) . unwrap () . label ("packet created/sent delta") . legend (| (x , y) | PathElement :: new (vec ! [(x , y) , (x + 20 , y)] , BLACK)) ; if params . display_legend { chart . configure_series_labels () . label_font (chart_label_style (& params . colors . caption)) . background_style (params . colors . fill . mix (0.8)) . border_style (params . colors . axis) . position (SeriesLabelPosition :: MiddleMiddle) . draw () . unwrap () ; } chart }
};
}
