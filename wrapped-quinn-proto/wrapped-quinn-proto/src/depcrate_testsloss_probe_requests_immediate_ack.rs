// Generated macro for loss_probe_requests_immediate_ack (function)
macro_rules! Depcrate_testsloss_probe_requests_immediate_ack {
() => {
// Module: crate::tests
// Provides: {"loss_probe_requests_immediate_ack"}
// Dependencies: {}
# [test] fn loss_probe_requests_immediate_ack () { let _guard = subscribe () ; let mut pair = Pair :: default () ; let (client_ch , _) = pair . connect () ; pair . drive () ; let stats_after_connect = pair . client_conn_mut (client_ch) . stats () ; let default_mtu = mem :: replace (& mut pair . mtu , 0) ; pair . client_conn_mut (client_ch) . ping () ; pair . drive_client () ; pair . mtu = default_mtu ; pair . drive () ; let stats_after_recovery = pair . client_conn_mut (client_ch) . stats () ; assert_eq ! (stats_after_recovery . frame_tx . immediate_ack - stats_after_connect . frame_tx . immediate_ack , 2) ; }
};
}
