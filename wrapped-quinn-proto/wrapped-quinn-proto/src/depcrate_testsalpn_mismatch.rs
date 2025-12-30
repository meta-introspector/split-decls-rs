// Generated macro for alpn_mismatch (function)
macro_rules! Depcrate_testsalpn_mismatch {
() => {
// Module: crate::tests
// Provides: {"alpn_mismatch"}
// Dependencies: {}
# [test] fn alpn_mismatch () { let _guard = subscribe () ; let server_config = ServerConfig :: with_crypto (Arc :: new (server_crypto_with_alpn (vec ! ["foo" . into () , "bar" . into () , "baz" . into () ,]))) ; let mut pair = Pair :: new (Arc :: new (EndpointConfig :: default ()) , server_config) ; let client_ch = pair . begin_connect (ClientConfig :: new (Arc :: new (client_crypto_with_alpn (vec ! ["quux" . into () , "corge" . into () ,])))) ; pair . drive () ; assert_matches ! (pair . client_conn_mut (client_ch) . poll () , Some (Event :: ConnectionLost { reason : ConnectionError :: ConnectionClosed (err) }) if err . error_code == TransportErrorCode :: crypto (0x78)) ; }
};
}
