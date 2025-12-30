// Generated macro for impl_24 (impl)
macro_rules! Depcrate_accept_opensslimpl_24 {
() => {
// Module: crate::accept::openssl
// Provides: {"impl_24"}
// Dependencies: {}
impl < IO : ActixStream > ServiceFactory < IO > for Acceptor { type Response = TlsStream < IO > ; type Error = TlsError < Error , Infallible > ; type Config = () ; type Service = AcceptorService ; type InitError = () ; type Future = FutReady < Result < Self :: Service , Self :: InitError > > ; fn new_service (& self , _ : ()) -> Self :: Future { let res = MAX_CONN_COUNTER . with (| conns | { Ok (AcceptorService { acceptor : self . acceptor . clone () , conns : conns . clone () , handshake_timeout : self . handshake_timeout , }) }) ; ready (res) } }
};
}
