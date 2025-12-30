// Generated macro for DebouncedEvent (struct)
macro_rules! Depcrate_debouncer_fullDebouncedEvent {
() => {
// Module: crate::debouncer_full
// Provides: {"DebouncedEvent"}
// Dependencies: {}
# [doc = " A debounced event is emitted after a short delay."] # [derive (Debug , Clone , PartialEq , Eq)] pub struct DebouncedEvent { # [doc = " The original event."] pub event : Event , # [doc = " The time at which the event occurred."] pub time : Instant , }
};
}
