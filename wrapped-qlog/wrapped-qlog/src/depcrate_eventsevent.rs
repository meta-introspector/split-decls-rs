// Generated macro for Event (struct)
macro_rules! Depcrate_eventsEvent {
() => {
// Module: crate::events
// Provides: {"Event"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , Debug)] pub struct Event { pub time : f32 , # [serde (flatten)] pub data : EventData , # [serde (flatten)] pub ex_data : ExData , pub protocol_type : Option < String > , pub group_id : Option < String > , pub time_format : Option < TimeFormat > , # [serde (skip)] ty : EventType , }
};
}
