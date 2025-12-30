// Generated macro for impl_75 (impl)
macro_rules! Depcrate_accept_rustls_0_21impl_75 {
() => {
// Module: crate::accept::rustls_0_21
// Provides: {"impl_75"}
// Dependencies: {}
impl < IO : ActixStream > ServiceFactory < IO > for Acceptor { type Response = TlsStream < IO > ; type Error = TlsError < io :: Error , Infallible > ; type Config = () ; type Service = AcceptorService ; type InitError = () ; type Future = FutReady < Result < Self :: Service , Self :: InitError > > ; fn new_service (& self , _ : ()) -> Self :: Future { let res = MAX_CONN_COUNTER . with (| conns | { Ok (AcceptorService { acceptor : self . config . clone () . into () , conns : conns . clone () , handshake_timeout : self . handshake_timeout , }) }) ; ready (res) } }
};
}
