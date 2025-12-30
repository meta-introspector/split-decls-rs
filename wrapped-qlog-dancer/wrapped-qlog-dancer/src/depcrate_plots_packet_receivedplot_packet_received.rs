// Generated macro for plot_packet_received (function)
macro_rules! Depcrate_plots_packet_receivedplot_packet_received {
() => {
// Module: crate::plots::packet_received
// Provides: {"plot_packet_received"}
// Dependencies: {}
pub fn plot_packet_received (params : & PlotParameters , filename : & str , ss : & SeriesStore , ds : & Datastore , ty : & ChartOutputType ,) { let chart_config = ChartConfig { title : "packet-received" . into () , input_filename : filename . into () , clamp : params . clamp . clone () , app_proto : ds . application_proto , host : ds . host . clone () , session_id : ds . session_id , ty : ty . clone () , } ; chart_config . init_chart_dir () ; # [cfg (not (target_arch = "wasm32"))] let chart_path = chart_config . chart_filepath () ; # [cfg (not (target_arch = "wasm32"))] let root = make_chart_bitmap_area (& chart_path , params . chart_size , params . colors , params . chart_margin ,) ; # [cfg (target_arch = "wasm32")] let canvas_id : String = chart_config . canvas_id () . unwrap_or_default () ; # [cfg (target_arch = "wasm32")] let root = make_chart_canvas_area (& canvas_id , params . colors , params . chart_margin) ; let (raw_timings , _remainder) = root . split_vertically ((60) . percent ()) ; draw_packet_sent_received_plot (false , filename , params , ss , & raw_timings) ; }
};
}
