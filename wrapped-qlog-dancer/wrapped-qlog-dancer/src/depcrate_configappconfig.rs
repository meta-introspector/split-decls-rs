// Generated macro for AppConfig (struct)
macro_rules! Depcrate_configAppConfig {
() => {
// Module: crate::config
// Provides: {"AppConfig"}
// Dependencies: {}
# [derive (Debug)] pub struct AppConfig { pub file : String , pub filename : String , pub charts_dir : String , pub plot_conn_overview : bool , pub plot_pkt_sent : bool , pub plot_pkt_received : bool , pub plot_conn_flow_control : bool , pub plot_sparks : bool , pub plot_multiplex : bool , pub plot_pending : bool , pub sparks_layout : SparkPlotsParams , pub report_text : bool , pub report_omit_upload : bool , pub report_omit_priorities : bool , pub report_text_csv : bool , pub report_html : bool , pub dark_mode : bool , pub start : Option < f32 > , pub end : Option < f32 > , pub stream_y_max : Option < u64 > , pub cwnd_y_max : Option < u64 > , pub netlog_filter : HashSet < String > , pub qlog_wirefilter : Option < String > , pub stats_config : PrintStatsConfig , pub ignore_acks : bool , pub log_format : SerializationFormat , }
};
}
