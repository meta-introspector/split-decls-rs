// Generated macro for _check_kinds (function)
macro_rules! Depcrate_check_kinds {
() => {
// Module: crate
// Provides: {"_check_kinds"}
// Dependencies: {}
fn _check_kinds () { use std :: net :: TcpStream ; fn is_sync < T : Sync > () { } fn is_send < T : Send > () { } is_sync :: < Error > () ; is_send :: < Error > () ; is_sync :: < TlsConnectorBuilder > () ; is_send :: < TlsConnectorBuilder > () ; is_sync :: < TlsConnector > () ; is_send :: < TlsConnector > () ; is_sync :: < TlsAcceptorBuilder > () ; is_send :: < TlsAcceptorBuilder > () ; is_sync :: < TlsAcceptor > () ; is_send :: < TlsAcceptor > () ; is_sync :: < TlsStream < TcpStream > > () ; is_send :: < TlsStream < TcpStream > > () ; is_sync :: < MidHandshakeTlsStream < TcpStream > > () ; is_send :: < MidHandshakeTlsStream < TcpStream > > () ; }
};
}
