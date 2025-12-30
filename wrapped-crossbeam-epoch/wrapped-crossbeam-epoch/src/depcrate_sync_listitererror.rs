// Generated macro for IterError (enum)
macro_rules! Depcrate_sync_listIterError {
() => {
// Module: crate::sync::list
// Provides: {"IterError"}
// Dependencies: {}
# [doc = " An error that occurs during iteration over the list."] # [derive (PartialEq , Debug)] pub (crate) enum IterError { # [doc = " A concurrent thread modified the state of the list at the same place that this iterator"] # [doc = " was inspecting. Subsequent iteration will restart from the beginning of the list."] Stalled , }
};
}
