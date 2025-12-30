// Generated macro for migration (function)
macro_rules! Depcrate_testsmigration {
() => {
// Module: crate::tests
// Provides: {"migration"}
// Dependencies: {}
# [test] fn migration () { let _guard = subscribe () ; let mut pair = Pair :: default () ; let (client_ch , server_ch) = pair . connect () ; pair . drive () ; let client_stats_after_connect = pair . client_conn_mut (client_ch) . stats () ; pair . client . addr = SocketAddr :: new (Ipv4Addr :: new (127 , 0 , 0 , 1) . into () , CLIENT_PORTS . lock () . unwrap () . next () . unwrap () ,) ; pair . client_conn_mut (client_ch) . ping () ; pair . drive_client () ; pair . drive_server () ; assert_ne ! (pair . server_conn_mut (server_ch) . total_recvd () , 0) ; pair . drive () ; assert_matches ! (pair . client_conn_mut (client_ch) . poll () , None) ; assert_eq ! (pair . server_conn_mut (server_ch) . remote_address () , pair . client . addr) ; let client_stats_after_migrate = pair . client_conn_mut (client_ch) . stats () ; assert_eq ! (client_stats_after_migrate . frame_tx . ping - client_stats_after_connect . frame_tx . ping , 1) ; assert_eq ! (client_stats_after_migrate . frame_tx . immediate_ack - client_stats_after_connect . frame_tx . immediate_ack , 1) ; }
};
}
