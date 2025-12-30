// Generated macro for implicit_open (function)
macro_rules! Depcrate_testsimplicit_open {
() => {
// Module: crate::tests
// Provides: {"implicit_open"}
// Dependencies: {}
# [test] fn implicit_open () { let _guard = subscribe () ; let mut pair = Pair :: default () ; let (client_ch , server_ch) = pair . connect () ; let s1 = pair . client_streams (client_ch) . open (Dir :: Uni) . unwrap () ; let s2 = pair . client_streams (client_ch) . open (Dir :: Uni) . unwrap () ; pair . client_send (client_ch , s2) . write (b"hello") . unwrap () ; pair . drive () ; assert_matches ! (pair . server_conn_mut (server_ch) . poll () , Some (Event :: Stream (StreamEvent :: Opened { dir : Dir :: Uni }))) ; assert_eq ! (pair . server_streams (server_ch) . accept (Dir :: Uni) , Some (s1)) ; assert_eq ! (pair . server_streams (server_ch) . accept (Dir :: Uni) , Some (s2)) ; assert_eq ! (pair . server_streams (server_ch) . accept (Dir :: Uni) , None) ; }
};
}
