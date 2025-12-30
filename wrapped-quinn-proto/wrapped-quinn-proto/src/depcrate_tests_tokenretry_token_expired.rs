// Generated macro for retry_token_expired (function)
macro_rules! Depcrate_tests_tokenretry_token_expired {
() => {
// Module: crate::tests::token
// Provides: {"retry_token_expired"}
// Dependencies: {}
# [test] fn retry_token_expired () { let _guard = subscribe () ; let fake_time = Arc :: new (FakeTimeSource :: new ()) ; let retry_token_lifetime = Duration :: from_secs (1) ; let mut pair = Pair :: default () ; pair . server . handle_incoming = Box :: new (validate_incoming) ; let mut config = server_config () ; config . time_source (Arc :: clone (& fake_time) as _) . retry_token_lifetime (retry_token_lifetime) ; pair . server . set_server_config (Some (Arc :: new (config))) ; let client_ch = pair . begin_connect (client_config ()) ; pair . drive_client () ; pair . drive_server () ; pair . drive_client () ; fake_time . advance (retry_token_lifetime + Duration :: from_millis (1)) ; pair . drive () ; assert_matches ! (pair . client_conn_mut (client_ch) . poll () , Some (Event :: ConnectionLost { reason : ConnectionError :: ConnectionClosed (err) }) if err . error_code == TransportErrorCode :: INVALID_TOKEN) ; assert_eq ! (pair . client . known_connections () , 0) ; assert_eq ! (pair . client . known_cids () , 0) ; assert_eq ! (pair . server . known_connections () , 0) ; assert_eq ! (pair . server . known_cids () , 0) ; }
};
}
