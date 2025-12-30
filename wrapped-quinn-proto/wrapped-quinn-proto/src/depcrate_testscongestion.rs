// Generated macro for congestion (function)
macro_rules! Depcrate_testscongestion {
() => {
// Module: crate::tests
// Provides: {"congestion"}
// Dependencies: {}
# [test] fn congestion () { let _guard = subscribe () ; let mut pair = Pair :: default () ; let (client_ch , _) = pair . connect () ; const TARGET : u64 = 2048 ; assert ! (pair . client_conn_mut (client_ch) . congestion_window () > TARGET) ; let s = pair . client_streams (client_ch) . open (Dir :: Uni) . unwrap () ; while pair . client_conn_mut (client_ch) . congestion_window () > TARGET { let n = pair . client_send (client_ch , s) . write (& [42 ; 1024]) . unwrap () ; assert_eq ! (n , 1024) ; pair . drive_client () ; } pair . drive () ; assert ! (pair . client_conn_mut (client_ch) . congestion_window () >= TARGET) ; pair . client_send (client_ch , s) . write (& [42 ; 1024]) . unwrap () ; }
};
}
