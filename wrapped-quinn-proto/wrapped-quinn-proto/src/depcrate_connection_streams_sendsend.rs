// Generated macro for Send (struct)
macro_rules! Depcrate_connection_streams_sendSend {
() => {
// Module: crate::connection::streams::send
// Provides: {"Send"}
// Dependencies: {}
# [derive (Debug)] pub (super) struct Send { pub (super) max_data : u64 , pub (super) state : SendState , pub (super) pending : SendBuffer , pub (super) priority : i32 , # [doc = " Whether a frame containing a FIN bit must be transmitted, even if we don't have any new data"] pub (super) fin_pending : bool , # [doc = " Whether this stream is in the `connection_blocked` list of `Streams`"] pub (super) connection_blocked : bool , # [doc = " The reason the peer wants us to stop, if `STOP_SENDING` was received"] pub (super) stop_reason : Option < VarInt > , }
};
}
