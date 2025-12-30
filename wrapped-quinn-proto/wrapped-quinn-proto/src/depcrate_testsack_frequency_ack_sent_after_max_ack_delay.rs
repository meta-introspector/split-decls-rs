// Generated macro for ack_frequency_ack_sent_after_max_ack_delay (function)
macro_rules! Depcrate_testsack_frequency_ack_sent_after_max_ack_delay {
() => {
// Module: crate::tests
// Provides: {"ack_frequency_ack_sent_after_max_ack_delay"}
// Dependencies: {}
# [test] fn ack_frequency_ack_sent_after_max_ack_delay () { let _guard = subscribe () ; let max_ack_delay = Duration :: from_millis (30) ; let (mut pair , client_ch , server_ch) = setup_ack_frequency_test (max_ack_delay) ; pair . client_conn_mut (client_ch) . ping () ; pair . drive_client () ; pair . time += pair . latency ; let server_stats_before = pair . server_conn_mut (server_ch) . stats () ; pair . drive_server () ; let server_stats_after = pair . server_conn_mut (server_ch) . stats () ; assert_eq ! (server_stats_after . frame_rx . ping - server_stats_before . frame_rx . ping , 1) ; assert_eq ! (server_stats_after . frame_tx . acks - server_stats_before . frame_tx . acks , 0) ; pair . time += max_ack_delay ; let server_stats_before = pair . server_conn_mut (server_ch) . stats () ; pair . drive_server () ; let server_stats_after = pair . server_conn_mut (server_ch) . stats () ; assert_eq ! (server_stats_after . frame_rx . ping - server_stats_before . frame_rx . ping , 0) ; assert_eq ! (server_stats_after . frame_tx . acks - server_stats_before . frame_tx . acks , 1) ; }
};
}
