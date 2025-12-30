// Generated macro for datagram_send_recv (function)
macro_rules! Depcrate_testsdatagram_send_recv {
() => {
// Module: crate::tests
// Provides: {"datagram_send_recv"}
// Dependencies: {}
# [test] fn datagram_send_recv () { let _guard = subscribe () ; let mut pair = Pair :: default () ; let (client_ch , server_ch) = pair . connect () ; assert_matches ! (pair . server_conn_mut (server_ch) . poll () , None) ; assert_matches ! (pair . client_datagrams (client_ch) . max_size () , Some (x) if x > 0) ; const DATA : & [u8] = b"whee" ; pair . client_datagrams (client_ch) . send (DATA . into () , true) . unwrap () ; pair . drive () ; assert_matches ! (pair . server_conn_mut (server_ch) . poll () , Some (Event :: DatagramReceived)) ; assert_eq ! (pair . server_datagrams (server_ch) . recv () . unwrap () , DATA) ; assert_matches ! (pair . server_datagrams (server_ch) . recv () , None) ; }
};
}
