// Generated macro for new_qlog_dancer (function)
macro_rules! Depcrate_webnew_qlog_dancer {
() => {
// Module: crate::web
// Provides: {"new_qlog_dancer"}
// Dependencies: {}
# [wasm_bindgen] # [doc = " Creates a QlogDancerWeb object to interact with logs."] # [doc = ""] # [doc = " The `display_name` parameter is reflected into drawn plots."] pub fn new_qlog_dancer (display_name : & str) -> QlogDancerWeb { let ds = Datastore { total_sent_stream_frame_count : 0 , vantage_point : VantagePoint :: Server , .. Default :: default () } ; QlogDancerWeb { ds , ss : None , display_name : display_name . into () , partial : vec ! [] , log_info : None , qlog_events : vec ! [] , mp_chart_info : None , cc_chart_info : None , rtt_chart_info : None , fc_chart_info : None , pkt_rx_chart_info : None , pkt_tx_chart_info : None , pkt_tx_counts_chart_info : None , pkt_tx_delta_chart_info : None , pkt_tx_pacing_chart_info : None , } }
};
}
