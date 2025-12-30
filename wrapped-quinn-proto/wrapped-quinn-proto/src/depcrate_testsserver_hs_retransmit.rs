// Generated macro for server_hs_retransmit (function)
macro_rules! Depcrate_testsserver_hs_retransmit {
() => {
// Module: crate::tests
// Provides: {"server_hs_retransmit"}
// Dependencies: {}
# [test] fn server_hs_retransmit () { let _guard = subscribe () ; let mut pair = Pair :: default () ; let client_ch = pair . begin_connect (client_config ()) ; pair . step () ; assert ! (! pair . client . inbound . is_empty ()) ; pair . client . inbound . clear () ; pair . drive () ; assert_matches ! (pair . client_conn_mut (client_ch) . poll () , Some (Event :: HandshakeDataReady)) ; assert_matches ! (pair . client_conn_mut (client_ch) . poll () , Some (Event :: Connected)) ; }
};
}
