// Generated macro for AckFrequency (struct)
macro_rules! Depcrate_frameAckFrequency {
() => {
// Module: crate::frame
// Provides: {"AckFrequency"}
// Dependencies: {}
# [derive (Debug , Copy , Clone , PartialEq , Eq)] pub (crate) struct AckFrequency { pub (crate) sequence : VarInt , pub (crate) ack_eliciting_threshold : VarInt , pub (crate) request_max_ack_delay : VarInt , pub (crate) reordering_threshold : VarInt , }
};
}
