// Generated macro for impl_289 (impl)
macro_rules! Depcrate_proto_connectionimpl_289 {
() => {
// Module: crate::proto::connection
// Provides: {"impl_289"}
// Dependencies: {}
impl < T , B > Connection < T , client :: Peer , B > where T : AsyncRead + AsyncWrite , B : Buf , { pub (crate) fn streams (& self) -> & Streams < B , client :: Peer > { & self . inner . streams } }
};
}
