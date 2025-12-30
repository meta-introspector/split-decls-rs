// Generated macro for Http3EventType (enum)
macro_rules! Depcrate_events_h3Http3EventType {
() => {
// Module: crate::events::h3
// Provides: {"Http3EventType"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Clone , Copy , PartialEq , Eq , Debug)] # [serde (rename_all = "snake_case")] pub enum Http3EventType { ParametersSet , ParametersRestored , StreamTypeSet , FrameCreated , FrameParsed , PushResolved , }
};
}
