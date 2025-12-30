// Generated macro for client_ca_list (function)
macro_rules! Depcrate_ssl_testclient_ca_list {
() => {
// Module: crate::ssl::test
// Provides: {"client_ca_list"}
// Dependencies: {}
# [test] fn client_ca_list () { let names = X509Name :: load_client_ca_file ("test/root-ca.pem") . unwrap () ; assert_eq ! (names . len () , 1) ; let mut ctx = SslContext :: builder (SslMethod :: tls ()) . unwrap () ; ctx . set_client_ca_list (names) ; }
};
}
