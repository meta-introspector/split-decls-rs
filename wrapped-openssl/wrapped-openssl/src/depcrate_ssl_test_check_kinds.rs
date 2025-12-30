// Generated macro for _check_kinds (function)
macro_rules! Depcrate_ssl_test_check_kinds {
() => {
// Module: crate::ssl::test
// Provides: {"_check_kinds"}
// Dependencies: {}
fn _check_kinds () { fn is_send < T : Send > () { } fn is_sync < T : Sync > () { } is_send :: < SslStream < TcpStream > > () ; is_sync :: < SslStream < TcpStream > > () ; }
};
}
