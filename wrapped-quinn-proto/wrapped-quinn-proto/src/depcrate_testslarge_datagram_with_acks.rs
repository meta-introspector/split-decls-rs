// Generated macro for large_datagram_with_acks (function)
macro_rules! Depcrate_testslarge_datagram_with_acks {
() => {
// Module: crate::tests
// Provides: {"large_datagram_with_acks"}
// Dependencies: {}
# [doc = " Verify that a large application datagram is sent successfully when an ACK frame too large to fit"] # [doc = " alongside it is also queued, in exactly 2 UDP datagrams."] # [test] fn large_datagram_with_acks () { let _guard = subscribe () ; let mut pair = Pair :: default () ; let (client_ch , server_ch) = pair . connect () ; for _ in 0 .. 10 { pair . server_conn_mut (server_ch) . ping () ; pair . drive_server () ; pair . client . inbound . pop_back () ; pair . server_conn_mut (server_ch) . ping () ; pair . drive_server () ; } let max_size = pair . client_datagrams (client_ch) . max_size () . unwrap () ; let msg = Bytes :: from (vec ! [0 ; max_size]) ; pair . client_datagrams (client_ch) . send (msg . clone () , true) . unwrap () ; let initial_datagrams = pair . client_conn_mut (client_ch) . stats () . udp_tx . datagrams ; pair . drive () ; let final_datagrams = pair . client_conn_mut (client_ch) . stats () . udp_tx . datagrams ; assert_eq ! (pair . server_datagrams (server_ch) . recv () . unwrap () , msg) ; assert_eq ! (final_datagrams - initial_datagrams , 2) ; }
};
}
