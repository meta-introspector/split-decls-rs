// Generated macro for Http3FrameTypeName (enum)
macro_rules! Depcrate_events_h3Http3FrameTypeName {
() => {
// Module: crate::events::h3
// Provides: {"Http3FrameTypeName"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug)] # [serde (rename_all = "snake_case")] pub enum Http3FrameTypeName { Data , Headers , CancelPush , Settings , PushPromise , Goaway , MaxPushId , DuplicatePush , Reserved , Unknown , }
};
}
