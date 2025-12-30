// Generated macro for RawInfo (struct)
macro_rules! Depcrate_eventsRawInfo {
() => {
// Module: crate::events
// Provides: {"RawInfo"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug , Default)] pub struct RawInfo { pub length : Option < u64 > , pub payload_length : Option < u64 > , pub data : Option < Bytes > , }
};
}
