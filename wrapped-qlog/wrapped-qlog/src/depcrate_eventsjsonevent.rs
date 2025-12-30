// Generated macro for JsonEvent (struct)
macro_rules! Depcrate_eventsJsonEvent {
() => {
// Module: crate::events
// Provides: {"JsonEvent"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Clone , Debug)] pub struct JsonEvent { pub time : f32 , # [serde (skip)] pub importance : EventImportance , pub name : String , pub data : serde_json :: Value , }
};
}
