// Generated macro for CongestionStateUpdated (struct)
macro_rules! Depcrate_events_quicCongestionStateUpdated {
() => {
// Module: crate::events::quic
// Provides: {"CongestionStateUpdated"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug , Default)] pub struct CongestionStateUpdated { pub old : Option < String > , pub new : String , pub trigger : Option < CongestionStateUpdatedTrigger > , }
};
}
