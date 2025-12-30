// Generated macro for RecvStream (struct)
macro_rules! Depcrate_connection_streamsRecvStream {
() => {
// Module: crate::connection::streams
// Provides: {"RecvStream"}
// Dependencies: {}
# [doc = " Access to streams"] pub struct RecvStream < 'a > { pub (super) id : StreamId , pub (super) state : & 'a mut StreamsState , pub (super) pending : & 'a mut Retransmits , }
};
}
