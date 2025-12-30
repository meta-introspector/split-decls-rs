// Generated macro for QpackStreamStateUpdated (struct)
macro_rules! Depcrate_events_qpackQpackStreamStateUpdated {
() => {
// Module: crate::events::qpack
// Provides: {"QpackStreamStateUpdated"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug)] pub struct QpackStreamStateUpdated { pub stream_id : u64 , pub state : QpackStreamState , }
};
}
