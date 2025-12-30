// Generated macro for Peer (trait)
macro_rules! Depcrate_proto_peerPeer {
() => {
// Module: crate::proto::peer
// Provides: {"Peer"}
// Dependencies: {}
# [doc = " Either a Client or a Server"] pub (crate) trait Peer { # [doc = " Message type polled from the transport"] type Poll : fmt :: Debug ; const NAME : & 'static str ; fn r#dyn () -> Dyn ; fn convert_poll_message (pseudo : Pseudo , fields : HeaderMap , stream_id : StreamId ,) -> Result < Self :: Poll , Error > ; }
};
}
