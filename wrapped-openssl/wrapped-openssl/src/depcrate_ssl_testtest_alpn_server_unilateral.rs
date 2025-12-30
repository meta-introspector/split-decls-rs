// Generated macro for test_alpn_server_unilateral (function)
macro_rules! Depcrate_ssl_testtest_alpn_server_unilateral {
() => {
// Module: crate::ssl::test
// Provides: {"test_alpn_server_unilateral"}
// Dependencies: {}
# [test] fn test_alpn_server_unilateral () { let server = Server :: builder () . build () ; let mut client = server . client () ; client . ctx () . set_alpn_protos (b"\x06http/2") . unwrap () ; let s = client . connect () ; assert_eq ! (None , s . ssl () . selected_alpn_protocol ()) ; }
};
}
