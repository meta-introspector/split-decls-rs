// Generated macro for StreamEvent (struct)
macro_rules! Depcrate_actions_h3StreamEvent {
() => {
// Module: crate::actions::h3
// Provides: {"StreamEvent"}
// Dependencies: {}
# [doc = " A response event, received over a stream, which will terminate the wait"] # [doc = " period."] # [doc = ""] # [doc = " See [StreamEventType] for the types of events."] # [derive (Copy , Clone , Debug , PartialEq , Eq , Hash , Serialize , Deserialize)] # [serde (rename = "snake_case")] pub struct StreamEvent { pub stream_id : u64 , # [serde (rename = "type")] pub event_type : StreamEventType , }
};
}
