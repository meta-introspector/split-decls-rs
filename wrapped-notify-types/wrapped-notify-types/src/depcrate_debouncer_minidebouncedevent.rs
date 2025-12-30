// Generated macro for DebouncedEvent (struct)
macro_rules! Depcrate_debouncer_miniDebouncedEvent {
() => {
// Module: crate::debouncer_mini
// Provides: {"DebouncedEvent"}
// Dependencies: {}
# [doc = " A debounced event."] # [doc = ""] # [doc = " Does not emit any specific event type on purpose, only distinguishes between an any event and a continuous any event."] # [derive (Clone , Debug , Eq , Hash , PartialEq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct DebouncedEvent { # [doc = " Event path"] pub path : PathBuf , # [doc = " Event kind"] pub kind : DebouncedEventKind , }
};
}
