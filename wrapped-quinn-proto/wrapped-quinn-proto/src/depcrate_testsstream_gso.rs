// Generated macro for stream_gso (function)
macro_rules! Depcrate_testsstream_gso {
() => {
// Module: crate::tests
// Provides: {"stream_gso"}
// Dependencies: {}
# [test] fn stream_gso () { let _guard = subscribe () ; let mut pair = Pair :: default () ; let (client_ch , _) = pair . connect () ; let s = pair . client_streams (client_ch) . open (Dir :: Uni) . unwrap () ; let initial_ios = pair . client_conn_mut (client_ch) . stats () . udp_tx . ios ; info ! ("sending") ; for _ in 0 .. 20 { pair . client_send (client_ch , s) . write (& [0 ; 1024]) . unwrap () ; } pair . client_send (client_ch , s) . finish () . unwrap () ; pair . drive () ; let final_ios = pair . client_conn_mut (client_ch) . stats () . udp_tx . ios ; assert_eq ! (final_ios - initial_ios , 2) ; }
};
}
