// Generated macro for alpn_google_none (function)
macro_rules! Depcrate_testalpn_google_none {
() => {
// Module: crate::test
// Provides: {"alpn_google_none"}
// Dependencies: {}
# [test] # [cfg (feature = "alpn")] fn alpn_google_none () { let builder = p ! (TlsConnector :: new ()) ; let s = p ! (TcpStream :: connect ("google.com:443")) ; let socket = p ! (builder . connect ("google.com" , s)) ; let alpn = p ! (socket . negotiated_alpn ()) ; assert_eq ! (alpn , None) ; }
};
}
