// Generated macro for MouseEvent (struct)
macro_rules! Depcrate_eventMouseEvent {
() => {
// Module: crate::event
// Provides: {"MouseEvent"}
// Dependencies: {}
# [doc = " Represents a mouse event."] # [doc = ""] # [doc = " # Platform-specific Notes"] # [doc = ""] # [doc = " ## Mouse Buttons"] # [doc = ""] # [doc = " Some platforms/terminals do not report mouse button for the"] # [doc = " `MouseEventKind::Up` and `MouseEventKind::Drag` events. `MouseButton::Left`"] # [doc = " is returned if we don't know which button was used."] # [doc = ""] # [doc = " ## Key Modifiers"] # [doc = ""] # [doc = " Some platforms/terminals does not report all key modifiers"] # [doc = " combinations for all mouse event types. For example - macOS reports"] # [doc = " `Ctrl` + left mouse button click as a right mouse button click."] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [derive (Debug , PartialOrd , Ord , PartialEq , Eq , Clone , Copy , Hash)] pub struct MouseEvent { # [doc = " The kind of mouse event that was caused."] pub kind : MouseEventKind , # [doc = " The column that the event occurred on."] pub column : u16 , # [doc = " The row that the event occurred on."] pub row : u16 , # [doc = " The key modifiers active when the event occurred."] pub modifiers : KeyModifiers , }
};
}
