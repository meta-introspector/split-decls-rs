// Generated macro for connector_invalid_no_hostname_verification (function)
macro_rules! Depcrate_ssl_testconnector_invalid_no_hostname_verification {
() => {
// Module: crate::ssl::test
// Provides: {"connector_invalid_no_hostname_verification"}
// Dependencies: {}
# [test] fn connector_invalid_no_hostname_verification () { let server = Server :: builder () . build () ; let mut connector = SslConnector :: builder (SslMethod :: tls ()) . unwrap () ; connector . set_ca_file ("test/root-ca.pem") . unwrap () ; let s = server . connect_tcp () ; let mut s = connector . build () . configure () . unwrap () . verify_hostname (false) . connect ("bogus.com" , s) . unwrap () ; s . read_exact (& mut [0]) . unwrap () ; }
};
}
