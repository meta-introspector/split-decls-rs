// Generated macro for ParsedEventMask (struct)
macro_rules! Depcrate_eventsParsedEventMask {
() => {
// Module: crate::events
// Provides: {"ParsedEventMask"}
// Dependencies: {}
# [doc = " A struct that provides structured access to event masks"] # [doc = " returned from reading an event from an inotify fd"] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct ParsedEventMask { # [doc = " The kind of event that occurred"] # [doc = ""] # [doc = " Inotify events that come from the kernel have"] # [doc = " exactly 0 or 1 of the flags associated with the"] # [doc = " event type set."] pub kind : Option < EventKind > , # [doc = " The auxiliary flags about the event"] pub auxiliary_flags : EventAuxiliaryFlags , }
};
}
