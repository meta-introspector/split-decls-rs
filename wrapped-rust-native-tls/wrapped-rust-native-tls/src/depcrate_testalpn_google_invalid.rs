// Generated macro for alpn_google_invalid (function)
macro_rules! Depcrate_testalpn_google_invalid {
() => {
// Module: crate::test
// Provides: {"alpn_google_invalid"}
// Dependencies: {}
# [test] # [cfg (feature = "alpn")] fn alpn_google_invalid () { let builder = p ! (TlsConnector :: builder () . request_alpns (& ["h2c"]) . build ()) ; let s = p ! (TcpStream :: connect ("google.com:443")) ; let socket = p ! (builder . connect ("google.com" , s)) ; let alpn = p ! (socket . negotiated_alpn ()) ; assert_eq ! (alpn , None) ; }
};
}
