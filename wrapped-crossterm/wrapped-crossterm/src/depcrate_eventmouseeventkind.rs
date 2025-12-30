// Generated macro for MouseEventKind (enum)
macro_rules! Depcrate_eventMouseEventKind {
() => {
// Module: crate::event
// Provides: {"MouseEventKind"}
// Dependencies: {}
# [doc = " A mouse event kind."] # [doc = ""] # [doc = " # Platform-specific Notes"] # [doc = ""] # [doc = " ## Mouse Buttons"] # [doc = ""] # [doc = " Some platforms/terminals do not report mouse button for the"] # [doc = " `MouseEventKind::Up` and `MouseEventKind::Drag` events. `MouseButton::Left`"] # [doc = " is returned if we don't know which button was used."] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [cfg_attr (feature = "derive-more" , derive (IsVariant))] # [derive (Debug , PartialOrd , Ord , PartialEq , Eq , Clone , Copy , Hash)] pub enum MouseEventKind { # [doc = " Pressed mouse button. Contains the button that was pressed."] Down (MouseButton) , # [doc = " Released mouse button. Contains the button that was released."] Up (MouseButton) , # [doc = " Moved the mouse cursor while pressing the contained mouse button."] Drag (MouseButton) , # [doc = " Moved the mouse cursor while not pressing a mouse button."] Moved , # [doc = " Scrolled mouse wheel downwards (towards the user)."] ScrollDown , # [doc = " Scrolled mouse wheel upwards (away from the user)."] ScrollUp , # [doc = " Scrolled mouse wheel left (mostly on a laptop touchpad)."] ScrollLeft , # [doc = " Scrolled mouse wheel right (mostly on a laptop touchpad)."] ScrollRight , }
};
}
