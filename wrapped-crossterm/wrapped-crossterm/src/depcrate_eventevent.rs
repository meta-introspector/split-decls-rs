// Generated macro for Event (enum)
macro_rules! Depcrate_eventEvent {
() => {
// Module: crate::event
// Provides: {"Event"}
// Dependencies: {}
# [doc = " Represents an event."] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [cfg_attr (feature = "derive-more" , derive (IsVariant))] # [cfg_attr (not (feature = "bracketed-paste") , derive (Copy))] # [derive (Debug , PartialOrd , Ord , PartialEq , Eq , Clone , Hash)] pub enum Event { # [doc = " The terminal gained focus"] FocusGained , # [doc = " The terminal lost focus"] FocusLost , # [doc = " A single key event with additional pressed modifiers."] Key (KeyEvent) , # [doc = " A single mouse event with additional pressed modifiers."] Mouse (MouseEvent) , # [doc = " A string that was pasted into the terminal. Only emitted if bracketed paste has been"] # [doc = " enabled."] # [cfg (feature = "bracketed-paste")] Paste (String) , # [doc = " A resize event with new dimensions after resize (columns, rows)."] # [doc = " **Note** that resize events can occur in batches."] Resize (u16 , u16) , }
};
}
