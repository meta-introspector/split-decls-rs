// Generated macro for high_latency_handshake (function)
macro_rules! Depcrate_testshigh_latency_handshake {
() => {
// Module: crate::tests
// Provides: {"high_latency_handshake"}
// Dependencies: {}
# [test] fn high_latency_handshake () { let _guard = subscribe () ; let mut pair = Pair :: default () ; pair . latency = Duration :: from_micros (200 * 1000) ; let (client_ch , server_ch) = pair . connect () ; assert_eq ! (pair . client_conn_mut (client_ch) . bytes_in_flight () , 0) ; assert_eq ! (pair . server_conn_mut (server_ch) . bytes_in_flight () , 0) ; assert ! (pair . client_conn_mut (client_ch) . using_ecn ()) ; assert ! (pair . server_conn_mut (server_ch) . using_ecn ()) ; }
};
}
