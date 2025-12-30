// Generated macro for DebounceEventResult (type)
macro_rules! DepcrateDebounceEventResult {
() => {
// Module: crate
// Provides: {"DebounceEventResult"}
// Dependencies: {}
# [doc = " A result of debounced events."] # [doc = " Comes with either a vec of events or an immediate error."] pub type DebounceEventResult = Result < Vec < DebouncedEvent > , Error > ;
};
}
