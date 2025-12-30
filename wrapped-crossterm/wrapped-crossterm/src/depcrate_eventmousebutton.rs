// Generated macro for MouseButton (enum)
macro_rules! Depcrate_eventMouseButton {
() => {
// Module: crate::event
// Provides: {"MouseButton"}
// Dependencies: {}
# [doc = " Represents a mouse button."] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [cfg_attr (feature = "derive-more" , derive (IsVariant))] # [derive (Debug , PartialOrd , Ord , PartialEq , Eq , Clone , Copy , Hash)] pub enum MouseButton { # [doc = " Left mouse button."] Left , # [doc = " Right mouse button."] Right , # [doc = " Middle mouse button."] Middle , }
};
}
