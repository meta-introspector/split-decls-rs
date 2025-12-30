// Generated macro for SetError (enum)
macro_rules! Depcrate_settingsSetError {
() => {
// Module: crate::settings
// Provides: {"SetError"}
// Dependencies: {}
# [doc = " An error produced when changing a setting."] # [derive (Debug , PartialEq , Eq)] pub enum SetError { # [doc = " No setting by this name exists."] BadName (String) , # [doc = " Type mismatch for setting (e.g., setting an enum setting as a bool)."] BadType , # [doc = " This is not a valid value for this setting."] BadValue (String) , }
};
}
