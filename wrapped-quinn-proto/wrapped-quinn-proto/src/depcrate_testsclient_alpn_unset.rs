// Generated macro for client_alpn_unset (function)
macro_rules! Depcrate_testsclient_alpn_unset {
() => {
// Module: crate::tests
// Provides: {"client_alpn_unset"}
// Dependencies: {}
# [test] fn client_alpn_unset () { let _guard = subscribe () ; let server_config = ServerConfig :: with_crypto (Arc :: new (server_crypto_with_alpn (vec ! ["foo" . into () , "bar" . into () , "baz" . into () ,]))) ; let mut pair = Pair :: new (Arc :: new (EndpointConfig :: default ()) , server_config) ; let client_ch = pair . begin_connect (client_config ()) ; pair . drive () ; assert_matches ! (pair . client_conn_mut (client_ch) . poll () , Some (Event :: ConnectionLost { reason : ConnectionError :: ConnectionClosed (err) }) if err . error_code == TransportErrorCode :: crypto (0x78)) ; }
};
}
