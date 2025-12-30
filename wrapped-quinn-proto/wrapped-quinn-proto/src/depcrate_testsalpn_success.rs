// Generated macro for alpn_success (function)
macro_rules! Depcrate_testsalpn_success {
() => {
// Module: crate::tests
// Provides: {"alpn_success"}
// Dependencies: {}
# [test] fn alpn_success () { let _guard = subscribe () ; let server_config = ServerConfig :: with_crypto (Arc :: new (server_crypto_with_alpn (vec ! ["foo" . into () , "bar" . into () , "baz" . into () ,]))) ; let mut pair = Pair :: new (Arc :: new (EndpointConfig :: default ()) , server_config) ; let client_config = ClientConfig :: new (Arc :: new (client_crypto_with_alpn (vec ! ["bar" . into () , "quux" . into () , "corge" . into () ,]))) ; let client_ch = pair . begin_connect (client_config) ; pair . drive () ; let server_ch = pair . server . assert_accept () ; assert_matches ! (pair . server_conn_mut (server_ch) . poll () , Some (Event :: HandshakeDataReady)) ; assert_matches ! (pair . server_conn_mut (server_ch) . poll () , Some (Event :: Connected)) ; let hd = pair . client_conn_mut (client_ch) . crypto_session () . handshake_data () . unwrap () . downcast :: < crate :: crypto :: rustls :: HandshakeData > () . unwrap () ; assert_eq ! (hd . protocol . unwrap () , & b"bar" [..]) ; }
};
}
