// Generated macro for impl_290 (impl)
macro_rules! Depcrate_proto_connectionimpl_290 {
() => {
// Module: crate::proto::connection
// Provides: {"impl_290"}
// Dependencies: {}
impl < T , B > Connection < T , server :: Peer , B > where T : AsyncRead + AsyncWrite + Unpin , B : Buf , { pub fn next_incoming (& mut self) -> Option < StreamRef < B > > { self . inner . streams . next_incoming () } pub fn go_away_gracefully (& mut self) { if self . inner . go_away . is_going_away () { return ; } self . inner . as_dyn () . go_away (StreamId :: MAX , Reason :: NO_ERROR) ; self . inner . ping_pong . ping_shutdown () ; } }
};
}
