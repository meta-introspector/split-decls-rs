// Generated macro for connect_bad_hostname (function)
macro_rules! Depcrate_testconnect_bad_hostname {
() => {
// Module: crate::test
// Provides: {"connect_bad_hostname"}
// Dependencies: {}
# [test] fn connect_bad_hostname () { let builder = p ! (TlsConnector :: new ()) ; let s = p ! (TcpStream :: connect ("google.com:443")) ; builder . connect ("goggle.com" , s) . unwrap_err () ; }
};
}
