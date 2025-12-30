// Generated macro for AckedRanges (enum)
macro_rules! Depcrate_events_quicAckedRanges {
() => {
// Module: crate::events::quic
// Provides: {"AckedRanges"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug)] # [serde (untagged)] pub enum AckedRanges { Single (Vec < Vec < u64 > >) , Double (Vec < (u64 , u64) >) , }
};
}
