// Generated macro for plot_packet_sent (function)
macro_rules! Depcrate_plots_packet_sentplot_packet_sent {
() => {
// Module: crate::plots::packet_sent
// Provides: {"plot_packet_sent"}
// Dependencies: {}
pub fn plot_packet_sent (params : & PlotParameters , filename : & str , ss : & SeriesStore , ds : & Datastore , ty : & ChartOutputType ,) { let chart_config = make_chart_config ("packet-sent" , params , filename , ds , ty) ; chart_config . init_chart_dir () ; # [cfg (not (target_arch = "wasm32"))] let chart_path = chart_config . chart_filepath () ; # [cfg (not (target_arch = "wasm32"))] let root = make_chart_bitmap_area (& chart_path , params . chart_size , params . colors , params . chart_margin ,) ; # [cfg (target_arch = "wasm32")] let canvas_id : String = chart_config . canvas_id () . unwrap_or_default () ; # [cfg (target_arch = "wasm32")] let root = make_chart_canvas_area (& canvas_id , params . colors , params . chart_margin) ; let (raw_timings , remainder) = root . split_vertically ((33) . percent ()) ; let (counts , remainder) = remainder . split_vertically ((33) . percent ()) ; let (delta_timings , pacing_rate) = remainder . split_vertically ((50) . percent ()) ; draw_packet_sent_received_plot (true , filename , params , ss , & raw_timings) ; draw_packet_sent_lost_delivered_count_plot (params , ss , & counts) ; draw_delta_plot (params , ss , & delta_timings) ; draw_pacing_rate_plot (params , ss , ds , & pacing_rate) ; }
};
}
