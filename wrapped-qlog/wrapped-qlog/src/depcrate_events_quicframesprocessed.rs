// Generated macro for FramesProcessed (struct)
macro_rules! Depcrate_events_quicFramesProcessed {
() => {
// Module: crate::events::quic
// Provides: {"FramesProcessed"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Debug , Default)] pub struct FramesProcessed { pub frames : Vec < QuicFrame > , pub packet_number : Option < u64 > , }
};
}
