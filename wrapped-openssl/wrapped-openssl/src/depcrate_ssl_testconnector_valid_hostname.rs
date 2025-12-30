// Generated macro for connector_valid_hostname (function)
macro_rules! Depcrate_ssl_testconnector_valid_hostname {
() => {
// Module: crate::ssl::test
// Provides: {"connector_valid_hostname"}
// Dependencies: {}
# [test] fn connector_valid_hostname () { let server = Server :: builder () . build () ; let mut connector = SslConnector :: builder (SslMethod :: tls ()) . unwrap () ; connector . set_ca_file ("test/root-ca.pem") . unwrap () ; let s = server . connect_tcp () ; let mut s = connector . build () . connect ("foobar.com" , s) . unwrap () ; s . read_exact (& mut [0]) . unwrap () ; }
};
}
