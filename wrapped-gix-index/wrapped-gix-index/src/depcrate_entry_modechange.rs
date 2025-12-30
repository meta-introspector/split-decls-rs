// Generated macro for Change (enum)
macro_rules! Depcrate_entry_modeChange {
() => {
// Module: crate::entry::mode
// Provides: {"Change"}
// Dependencies: {}
# [doc = " A change of a [`Mode`]."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] pub enum Change { # [doc = " The type of mode changed, like symlink => file."] Type { # [doc = " The mode representing the new index type."] new_mode : Mode , } , # [doc = " The executable permission of this file has changed."] ExecutableBit , }
};
}
