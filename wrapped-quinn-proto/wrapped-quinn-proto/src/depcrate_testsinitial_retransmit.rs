// Generated macro for initial_retransmit (function)
macro_rules! Depcrate_testsinitial_retransmit {
() => {
// Module: crate::tests
// Provides: {"initial_retransmit"}
// Dependencies: {}
# [test] fn initial_retransmit () { let _guard = subscribe () ; let mut pair = Pair :: default () ; let client_ch = pair . begin_connect (client_config ()) ; pair . client . drive (pair . time , pair . server . addr) ; pair . client . outbound . clear () ; pair . drive () ; assert_matches ! (pair . client_conn_mut (client_ch) . poll () , Some (Event :: HandshakeDataReady)) ; assert_matches ! (pair . client_conn_mut (client_ch) . poll () , Some (Event :: Connected)) ; }
};
}
