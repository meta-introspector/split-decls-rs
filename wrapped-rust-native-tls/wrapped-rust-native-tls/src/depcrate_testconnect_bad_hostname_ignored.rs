// Generated macro for connect_bad_hostname_ignored (function)
macro_rules! Depcrate_testconnect_bad_hostname_ignored {
() => {
// Module: crate::test
// Provides: {"connect_bad_hostname_ignored"}
// Dependencies: {}
# [test] fn connect_bad_hostname_ignored () { let builder = p ! (TlsConnector :: builder () . danger_accept_invalid_hostnames (true) . build ()) ; let s = p ! (TcpStream :: connect ("google.com:443")) ; builder . connect ("goggle.com" , s) . unwrap () ; }
};
}
