// Generated macro for SendStream (struct)
macro_rules! Depcrate_connection_streamsSendStream {
() => {
// Module: crate::connection::streams
// Provides: {"SendStream"}
// Dependencies: {}
# [doc = " Access to streams"] pub struct SendStream < 'a > { pub (super) id : StreamId , pub (super) state : & 'a mut StreamsState , pub (super) pending : & 'a mut Retransmits , pub (super) conn_state : & 'a super :: State , }
};
}
