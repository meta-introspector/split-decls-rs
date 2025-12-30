// Generated macro for impl_16 (impl)
macro_rules! Depcrate_eventsimpl_16 {
() => {
// Module: crate::events
// Provides: {"impl_16"}
// Dependencies: {}
impl ParsedEventMask { # [doc = " Construct a `ParsedEventMask` from its component parts"] pub fn from_parts (kind : Option < EventKind > , auxiliary_flags : EventAuxiliaryFlags) -> Self { ParsedEventMask { kind , auxiliary_flags , } } # [doc = " Parse a raw event mask"] pub fn from_raw_event_mask (mask : EventMask) -> Result < Self , EventMaskParseError > { if mask . contains (EventMask :: Q_OVERFLOW) { return Err (EventMaskParseError :: QueueOverflow) ; } let kind = Option :: < EventKind > :: try_from (mask) ? ; let auxiliary_flags = EventAuxiliaryFlags :: from (mask) ; Ok (ParsedEventMask :: from_parts (kind , auxiliary_flags)) } }
};
}
