// Generated macro for DebouncedEventKind (enum)
macro_rules! Depcrate_debouncer_miniDebouncedEventKind {
() => {
// Module: crate::debouncer_mini
// Provides: {"DebouncedEventKind"}
// Dependencies: {}
# [doc = " A debounced event kind."] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] # [non_exhaustive] pub enum DebouncedEventKind { # [doc = " No precise events"] Any , # [doc = " Event but debounce timed out (for example continuous writes)"] AnyContinuous , }
};
}
