// Generated macro for connect_no_root_certs (function)
macro_rules! Depcrate_testconnect_no_root_certs {
() => {
// Module: crate::test
// Provides: {"connect_no_root_certs"}
// Dependencies: {}
# [test] fn connect_no_root_certs () { let builder = p ! (TlsConnector :: builder () . disable_built_in_roots (true) . build ()) ; let s = p ! (TcpStream :: connect ("google.com:443")) ; assert ! (builder . connect ("google.com" , s) . is_err ()) ; }
};
}
