// Generated macro for impl_291 (impl)
macro_rules! Depcrate_proto_connectionimpl_291 {
() => {
// Module: crate::proto::connection
// Provides: {"impl_291"}
// Dependencies: {}
impl < T , P , B > Drop for Connection < T , P , B > where P : Peer , B : Buf , { fn drop (& mut self) { let _ = self . inner . streams . recv_eof (true) ; } }
};
}
