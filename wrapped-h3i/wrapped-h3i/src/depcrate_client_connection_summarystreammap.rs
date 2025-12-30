// Generated macro for StreamMap (struct)
macro_rules! Depcrate_client_connection_summaryStreamMap {
() => {
// Module: crate::client::connection_summary
// Provides: {"StreamMap"}
// Dependencies: {}
# [doc = " A read-only aggregation of frames received over a connection, mapped to the"] # [doc = " stream ID over which they were received."] # [doc = ""] # [doc = " [`StreamMap`] also contains the [`CloseTriggerFrames`] for the connection so"] # [doc = " that its state can be updated as new frames are received."] # [derive (Clone , Debug , Default , Serialize)] pub struct StreamMap { stream_frame_map : HashMap < u64 , Vec < H3iFrame > > , close_trigger_frames : Option < CloseTriggerFrames > , }
};
}
