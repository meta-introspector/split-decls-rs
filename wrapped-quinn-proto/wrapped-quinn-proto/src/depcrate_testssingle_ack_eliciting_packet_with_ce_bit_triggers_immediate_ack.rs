// Generated macro for single_ack_eliciting_packet_with_ce_bit_triggers_immediate_ack (function)
macro_rules! Depcrate_testssingle_ack_eliciting_packet_with_ce_bit_triggers_immediate_ack {
() => {
// Module: crate::tests
// Provides: {"single_ack_eliciting_packet_with_ce_bit_triggers_immediate_ack"}
// Dependencies: {}
# [test] fn single_ack_eliciting_packet_with_ce_bit_triggers_immediate_ack () { let _guard = subscribe () ; let mut pair = Pair :: default_with_deterministic_pns () ; let (client_ch , _) = pair . connect_with (client_config_with_deterministic_pns ()) ; pair . drive () ; let stats_after_connect = pair . client_conn_mut (client_ch) . stats () ; let start = pair . time ; pair . client_conn_mut (client_ch) . ping () ; pair . congestion_experienced = true ; pair . drive_client () ; pair . congestion_experienced = false ; pair . drive_server () ; pair . drive_client () ; assert_eq ! (pair . time , start) ; let stats_after_ping = pair . client_conn_mut (client_ch) . stats () ; assert_eq ! (stats_after_ping . frame_tx . ping - stats_after_connect . frame_tx . ping , 1) ; assert_eq ! (stats_after_ping . frame_rx . acks - stats_after_connect . frame_rx . acks , 1) ; assert_eq ! (stats_after_ping . path . congestion_events - stats_after_connect . path . congestion_events , 1) ; }
};
}
