// Generated macro for ack_frequency_update_max_delay (function)
macro_rules! Depcrate_testsack_frequency_update_max_delay {
() => {
// Module: crate::tests
// Provides: {"ack_frequency_update_max_delay"}
// Dependencies: {}
# [test] fn ack_frequency_update_max_delay () { let _guard = subscribe () ; let (mut pair , client_ch , server_ch) = setup_ack_frequency_test (Duration :: from_millis (200)) ; assert_eq ! (pair . server_conn_mut (server_ch) . stats () . frame_rx . ack_frequency , 1) ; info ! ("first ping") ; pair . client_conn_mut (client_ch) . ping () ; pair . drive () ; assert_eq ! (pair . server_conn_mut (server_ch) . stats () . frame_rx . ack_frequency , 1) ; info ! ("delayed ping") ; pair . latency *= 10 ; pair . client_conn_mut (client_ch) . ping () ; pair . drive () ; assert ! (pair . server_conn_mut (server_ch) . stats () . frame_rx . ack_frequency >= 2) ; }
};
}
