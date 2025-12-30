// Generated macro for immediate_ack_triggers_ack (function)
macro_rules! Depcrate_testsimmediate_ack_triggers_ack {
() => {
// Module: crate::tests
// Provides: {"immediate_ack_triggers_ack"}
// Dependencies: {}
# [test] fn immediate_ack_triggers_ack () { let _guard = subscribe () ; let mut pair = Pair :: default_with_deterministic_pns () ; let (client_ch , _) = pair . connect_with (client_config_with_deterministic_pns ()) ; pair . drive () ; let acks_after_connect = pair . client_conn_mut (client_ch) . stats () . frame_rx . acks ; pair . client_conn_mut (client_ch) . immediate_ack () ; pair . drive_client () ; pair . drive_server () ; pair . drive_client () ; let acks_after_ping = pair . client_conn_mut (client_ch) . stats () . frame_rx . acks ; assert_eq ! (acks_after_ping - acks_after_connect , 1) ; }
};
}
