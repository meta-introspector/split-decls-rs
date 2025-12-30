// Generated macro for impl_9 (impl)
macro_rules! Depcrate_eventsimpl_9 {
() => {
// Module: crate::events
// Provides: {"impl_9"}
// Dependencies: {}
impl < 'a > Iterator for Events < 'a > { type Item = Event < & 'a OsStr > ; fn next (& mut self) -> Option < Self :: Item > { if self . pos < self . num_bytes { let (step , event) = Event :: from_buffer (self . fd . clone () , & self . buffer [self . pos ..]) ; self . pos += step ; Some (event) } else { None } } }
};
}
