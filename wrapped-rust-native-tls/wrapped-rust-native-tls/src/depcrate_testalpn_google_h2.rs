// Generated macro for alpn_google_h2 (function)
macro_rules! Depcrate_testalpn_google_h2 {
() => {
// Module: crate::test
// Provides: {"alpn_google_h2"}
// Dependencies: {}
# [test] # [cfg (feature = "alpn")] fn alpn_google_h2 () { let builder = p ! (TlsConnector :: builder () . request_alpns (& ["h2"]) . build ()) ; let s = p ! (TcpStream :: connect ("google.com:443")) ; let socket = p ! (builder . connect ("google.com" , s)) ; let alpn = p ! (socket . negotiated_alpn ()) ; assert_eq ! (alpn , Some (b"h2" . to_vec ())) ; }
};
}
