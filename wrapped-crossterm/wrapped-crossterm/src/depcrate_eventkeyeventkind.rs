// Generated macro for KeyEventKind (enum)
macro_rules! Depcrate_eventKeyEventKind {
() => {
// Module: crate::event
// Provides: {"KeyEventKind"}
// Dependencies: {}
# [doc = " Represents a keyboard event kind."] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [cfg_attr (feature = "derive-more" , derive (IsVariant))] # [derive (Debug , PartialOrd , Ord , PartialEq , Eq , Clone , Copy , Hash)] pub enum KeyEventKind { Press , Repeat , Release , }
};
}
