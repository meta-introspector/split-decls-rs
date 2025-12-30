// Generated macro for EventMaskParseError (enum)
macro_rules! Depcrate_eventsEventMaskParseError {
() => {
// Module: crate::events
// Provides: {"EventMaskParseError"}
// Dependencies: {}
# [doc = " An error that occured from parsing an raw event mask"] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum EventMaskParseError { # [doc = " More than one bit repesenting the event type was set"] TooManyBitsSet (EventMask) , # [doc = " The event is a signal that the kernels event queue overflowed"] QueueOverflow , }
};
}
