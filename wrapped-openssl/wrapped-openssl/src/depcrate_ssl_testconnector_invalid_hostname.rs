// Generated macro for connector_invalid_hostname (function)
macro_rules! Depcrate_ssl_testconnector_invalid_hostname {
() => {
// Module: crate::ssl::test
// Provides: {"connector_invalid_hostname"}
// Dependencies: {}
# [test] fn connector_invalid_hostname () { let mut server = Server :: builder () ; server . should_error () ; let server = server . build () ; let mut connector = SslConnector :: builder (SslMethod :: tls ()) . unwrap () ; connector . set_ca_file ("test/root-ca.pem") . unwrap () ; let s = server . connect_tcp () ; connector . build () . connect ("bogus.com" , s) . unwrap_err () ; }
};
}
