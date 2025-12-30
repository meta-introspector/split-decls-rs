// Generated macro for StreamState (enum)
macro_rules! Depcrate_events_quicStreamState {
() => {
// Module: crate::events::quic
// Provides: {"StreamState"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug)] # [serde (rename_all = "snake_case")] pub enum StreamState { Idle , Open , HalfClosedLocal , HalfClosedRemote , Closed , Ready , Send , DataSent , ResetSent , ResetReceived , Receive , SizeKnown , DataRead , ResetRead , DataReceived , Destroyed , }
};
}
