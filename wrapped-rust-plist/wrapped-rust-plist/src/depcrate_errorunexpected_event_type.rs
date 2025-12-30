// Generated macro for unexpected_event_type (function)
macro_rules! Depcrate_errorunexpected_event_type {
() => {
// Module: crate::error
// Provides: {"unexpected_event_type"}
// Dependencies: {}
# [cfg (feature = "serde")] pub (crate) fn unexpected_event_type (expected : EventKind , found : & Event) -> Error { let found = EventKind :: of_event (found) ; ErrorKind :: UnexpectedEventType { expected , found } . without_position () }
};
}
