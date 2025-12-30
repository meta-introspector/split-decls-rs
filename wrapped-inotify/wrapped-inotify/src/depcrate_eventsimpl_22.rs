// Generated macro for impl_22 (impl)
macro_rules! Depcrate_eventsimpl_22 {
() => {
// Module: crate::events
// Provides: {"impl_22"}
// Dependencies: {}
impl EventAuxiliaryFlags { # [doc = " Parse the auxiliary flags from a raw event mask"] pub fn from_raw_event_mask (mask : EventMask) -> Self { EventAuxiliaryFlags { ignored : mask . contains (EventMask :: IGNORED) , isdir : mask . contains (EventMask :: ISDIR) , unmount : mask . contains (EventMask :: UNMOUNT) , } } }
};
}
