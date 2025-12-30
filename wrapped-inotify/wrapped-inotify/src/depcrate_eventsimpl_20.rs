// Generated macro for impl_20 (impl)
macro_rules! Depcrate_eventsimpl_20 {
() => {
// Module: crate::events
// Provides: {"impl_20"}
// Dependencies: {}
impl TryFrom < EventMask > for Option < EventKind > { type Error = EventMaskParseError ; fn try_from (value : EventMask) -> Result < Self , Self :: Error > { EventKind :: from_raw_event_mask (value) } }
};
}
