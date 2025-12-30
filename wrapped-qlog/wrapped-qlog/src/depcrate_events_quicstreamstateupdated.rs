// Generated macro for StreamStateUpdated (struct)
macro_rules! Depcrate_events_quicStreamStateUpdated {
() => {
// Module: crate::events::quic
// Provides: {"StreamStateUpdated"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug)] pub struct StreamStateUpdated { pub stream_id : u64 , pub stream_type : Option < StreamType > , pub old : Option < StreamState > , pub new : StreamState , pub stream_side : Option < StreamSide > , }
};
}
