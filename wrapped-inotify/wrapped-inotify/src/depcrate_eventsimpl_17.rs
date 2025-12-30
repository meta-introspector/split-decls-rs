// Generated macro for impl_17 (impl)
macro_rules! Depcrate_eventsimpl_17 {
() => {
// Module: crate::events
// Provides: {"impl_17"}
// Dependencies: {}
impl TryFrom < EventMask > for ParsedEventMask { type Error = EventMaskParseError ; fn try_from (value : EventMask) -> Result < Self , Self :: Error > { Self :: from_raw_event_mask (value) } }
};
}
