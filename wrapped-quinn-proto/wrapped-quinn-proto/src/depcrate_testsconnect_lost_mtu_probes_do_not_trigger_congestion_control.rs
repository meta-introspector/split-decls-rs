// Generated macro for connect_lost_mtu_probes_do_not_trigger_congestion_control (function)
macro_rules! Depcrate_testsconnect_lost_mtu_probes_do_not_trigger_congestion_control {
() => {
// Module: crate::tests
// Provides: {"connect_lost_mtu_probes_do_not_trigger_congestion_control"}
// Dependencies: {}
# [test] fn connect_lost_mtu_probes_do_not_trigger_congestion_control () { let _guard = subscribe () ; let mut pair = Pair :: default () ; pair . mtu = 1200 ; let (client_ch , server_ch) = pair . connect () ; pair . drive () ; let client_stats = pair . client_conn_mut (client_ch) . stats () ; let server_stats = pair . server_conn_mut (server_ch) . stats () ; assert_eq ! (client_stats . path . sent_plpmtud_probes , 9) ; assert_eq ! (client_stats . path . lost_plpmtud_probes , 9) ; assert_eq ! (server_stats . path . sent_plpmtud_probes , 9) ; assert_eq ! (server_stats . path . lost_plpmtud_probes , 9) ; assert_eq ! (client_stats . path . congestion_events , 0) ; assert_eq ! (server_stats . path . congestion_events , 0) ; }
};
}
