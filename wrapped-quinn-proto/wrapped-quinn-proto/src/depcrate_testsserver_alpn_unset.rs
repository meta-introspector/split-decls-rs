// Generated macro for server_alpn_unset (function)
macro_rules! Depcrate_testsserver_alpn_unset {
() => {
// Module: crate::tests
// Provides: {"server_alpn_unset"}
// Dependencies: {}
# [test] fn server_alpn_unset () { let _guard = subscribe () ; let mut pair = Pair :: new (Arc :: new (EndpointConfig :: default ()) , server_config ()) ; let client_config = ClientConfig :: new (Arc :: new (client_crypto_with_alpn (vec ! ["foo" . into ()]))) ; let client_ch = pair . begin_connect (client_config) ; pair . drive () ; assert_matches ! (pair . client_conn_mut (client_ch) . poll () , Some (Event :: ConnectionLost { reason : ConnectionError :: ConnectionClosed (err) }) if err . error_code == TransportErrorCode :: crypto (0x78)) ; }
};
}
