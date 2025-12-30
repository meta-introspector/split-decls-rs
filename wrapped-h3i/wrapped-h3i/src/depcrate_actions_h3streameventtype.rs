// Generated macro for StreamEventType (enum)
macro_rules! Depcrate_actions_h3StreamEventType {
() => {
// Module: crate::actions::h3
// Provides: {"StreamEventType"}
// Dependencies: {}
# [doc = " Response that can terminate a wait period."] # [derive (Copy , Clone , Debug , PartialEq , Eq , Hash , Serialize , Deserialize)] # [serde (rename_all = "lowercase")] pub enum StreamEventType { # [doc = " A HEADERS frame was received."] Headers , # [doc = " A DATA frame was received."] Data , # [doc = " The stream was somehow finished, either by a RESET_STREAM frame or via"] # [doc = " the `fin` bit being set."] Finished , }
};
}
