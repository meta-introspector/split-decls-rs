// Generated macro for ModifierKeyCode (enum)
macro_rules! Depcrate_eventModifierKeyCode {
() => {
// Module: crate::event
// Provides: {"ModifierKeyCode"}
// Dependencies: {}
# [doc = " Represents a modifier key (as part of [`KeyCode::Modifier`])."] # [derive (Debug , PartialOrd , Ord , PartialEq , Eq , Clone , Copy , Hash)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum ModifierKeyCode { # [doc = " Left Shift key."] LeftShift , # [doc = " Left Control key. (Control on macOS, Ctrl on other platforms)"] LeftControl , # [doc = " Left Alt key. (Option on macOS, Alt on other platforms)"] LeftAlt , # [doc = " Left Super key. (Command on macOS, Windows on Windows, Super on other platforms)"] LeftSuper , # [doc = " Left Hyper key."] LeftHyper , # [doc = " Left Meta key."] LeftMeta , # [doc = " Right Shift key."] RightShift , # [doc = " Right Control key. (Control on macOS, Ctrl on other platforms)"] RightControl , # [doc = " Right Alt key. (Option on macOS, Alt on other platforms)"] RightAlt , # [doc = " Right Super key. (Command on macOS, Windows on Windows, Super on other platforms)"] RightSuper , # [doc = " Right Hyper key."] RightHyper , # [doc = " Right Meta key."] RightMeta , # [doc = " Iso Level3 Shift key."] IsoLevel3Shift , # [doc = " Iso Level5 Shift key."] IsoLevel5Shift , }
};
}
