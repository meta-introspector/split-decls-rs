// Generated macro for blackhole_after_mtu_change_repairs_itself (function)
macro_rules! Depcrate_testsblackhole_after_mtu_change_repairs_itself {
() => {
// Module: crate::tests
// Provides: {"blackhole_after_mtu_change_repairs_itself"}
// Dependencies: {}
# [test] fn blackhole_after_mtu_change_repairs_itself () { let _guard = subscribe () ; let mut pair = Pair :: default () ; pair . mtu = 1500 ; let (client_ch , server_ch) = pair . connect () ; pair . drive () ; assert_eq ! (pair . client_conn_mut (client_ch) . path_mtu () , 1452) ; assert_eq ! (pair . server_conn_mut (server_ch) . path_mtu () , 1452) ; pair . mtu = 1200 ; let payload = vec ! [42 ; 1300] ; let s = pair . client_streams (client_ch) . open (Dir :: Uni) . unwrap () ; pair . client_send (client_ch , s) . write (& payload) . unwrap () ; let out_of_bounds = pair . drive_bounded () ; if out_of_bounds { panic ! ("Connections never reached an idle state") ; } let recv = pair . server_recv (server_ch , s) ; let buf = stream_chunks (recv) ; assert_eq ! (buf . len () , 1300) ; let client_stats = pair . client_conn_mut (client_ch) . stats () ; assert ! (client_stats . path . lost_packets >= 3) ; assert ! (client_stats . path . congestion_events >= 3) ; assert_eq ! (client_stats . path . black_holes_detected , 1) ; }
};
}
