// Generated macro for WaitType (enum)
macro_rules! Depcrate_actions_h3WaitType {
() => {
// Module: crate::actions::h3
// Provides: {"WaitType"}
// Dependencies: {}
# [doc = " Configure the wait behavior for a connection."] # [serde_as] # [derive (Clone , Debug , PartialEq , Eq , Hash , Serialize , Deserialize)] pub enum WaitType { # [doc = " Wait for a time before firing the next action"] # [serde (rename = "duration")] WaitDuration (# [serde_as (as = "serde_with::DurationMilliSecondsWithFrac<f64>")] Duration ,) , # [doc = " Wait for some form of a response before firing the next action. This can"] # [doc = " be superseded in several cases:"] # [doc = " 1. The peer resets the specified stream."] # [doc = " 2. The peer sends a `fin` over the specified stream"] StreamEvent (StreamEvent) , }
};
}
