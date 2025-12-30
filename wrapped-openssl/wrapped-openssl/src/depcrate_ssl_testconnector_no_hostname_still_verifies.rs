// Generated macro for connector_no_hostname_still_verifies (function)
macro_rules! Depcrate_ssl_testconnector_no_hostname_still_verifies {
() => {
// Module: crate::ssl::test
// Provides: {"connector_no_hostname_still_verifies"}
// Dependencies: {}
# [test] fn connector_no_hostname_still_verifies () { let mut server = Server :: builder () ; server . should_error () ; let server = server . build () ; let connector = SslConnector :: builder (SslMethod :: tls ()) . unwrap () . build () ; let s = server . connect_tcp () ; assert ! (connector . configure () . unwrap () . verify_hostname (false) . connect ("fizzbuzz.com" , s) . is_err ()) ; }
};
}
