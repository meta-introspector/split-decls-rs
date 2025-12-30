// Generated macro for large_initial (function)
macro_rules! Depcrate_testslarge_initial {
() => {
// Module: crate::tests
// Provides: {"large_initial"}
// Dependencies: {}
# [test] fn large_initial () { let _guard = subscribe () ; let server_config = ServerConfig :: with_crypto (Arc :: new (server_crypto_with_alpn (vec ! [vec ! [0 , 0 , 0 , 42]]))) ; let mut pair = Pair :: new (Arc :: new (EndpointConfig :: default ()) , server_config) ; let client_crypto = client_crypto_with_alpn ((0 .. 1000u32) . map (| x | x . to_be_bytes () . to_vec ()) . collect ()) ; let cfg = ClientConfig :: new (Arc :: new (client_crypto)) ; let client_ch = pair . begin_connect (cfg) ; pair . drive () ; let server_ch = pair . server . assert_accept () ; assert_matches ! (pair . client_conn_mut (client_ch) . poll () , Some (Event :: HandshakeDataReady)) ; assert_matches ! (pair . client_conn_mut (client_ch) . poll () , Some (Event :: Connected)) ; assert_matches ! (pair . server_conn_mut (server_ch) . poll () , Some (Event :: HandshakeDataReady)) ; assert_matches ! (pair . server_conn_mut (server_ch) . poll () , Some (Event :: Connected)) ; }
};
}
