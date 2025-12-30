// Generated macro for connector_can_disable_verify (function)
macro_rules! Depcrate_ssl_testconnector_can_disable_verify {
() => {
// Module: crate::ssl::test
// Provides: {"connector_can_disable_verify"}
// Dependencies: {}
# [test] fn connector_can_disable_verify () { let server = Server :: builder () . build () ; let mut connector = SslConnector :: builder (SslMethod :: tls ()) . unwrap () ; connector . set_verify (SslVerifyMode :: NONE) ; let connector = connector . build () ; let s = server . connect_tcp () ; let mut s = connector . configure () . unwrap () . connect ("fizzbuzz.com" , s) . unwrap () ; s . read_exact (& mut [0]) . unwrap () ; }
};
}
