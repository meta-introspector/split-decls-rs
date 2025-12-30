// Generated macro for FrameStats (struct)
macro_rules! Depcrate_connection_statsFrameStats {
() => {
// Module: crate::connection::stats
// Provides: {"FrameStats"}
// Dependencies: {}
# [doc = " Number of frames transmitted or received of each frame type"] # [derive (Default , Copy , Clone)] # [non_exhaustive] # [allow (missing_docs)] pub struct FrameStats { pub acks : u64 , pub ack_frequency : u64 , pub crypto : u64 , pub connection_close : u64 , pub data_blocked : u64 , pub datagram : u64 , pub handshake_done : u8 , pub immediate_ack : u64 , pub max_data : u64 , pub max_stream_data : u64 , pub max_streams_bidi : u64 , pub max_streams_uni : u64 , pub new_connection_id : u64 , pub new_token : u64 , pub path_challenge : u64 , pub path_response : u64 , pub ping : u64 , pub reset_stream : u64 , pub retire_connection_id : u64 , pub stream_data_blocked : u64 , pub streams_blocked_bidi : u64 , pub streams_blocked_uni : u64 , pub stop_sending : u64 , pub stream : u64 , }
};
}
