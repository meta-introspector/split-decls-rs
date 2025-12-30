// Generated macro for Event (enum)
macro_rules! Depcrate_readerEvent {
() => {
// Module: crate::reader
// Provides: {"Event"}
// Dependencies: {}
# [doc = " Represents the format of the read event."] # [allow (clippy :: large_enum_variant)] # [derive (Clone , Debug)] pub enum Event { # [doc = " A native qlog event type."] Qlog (crate :: events :: Event) , Json (crate :: events :: JsonEvent) , }
};
}
