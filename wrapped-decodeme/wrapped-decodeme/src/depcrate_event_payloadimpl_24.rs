// Generated macro for impl_24 (impl)
macro_rules! Depcrate_event_payloadimpl_24 {
() => {
// Module: crate::event_payload
// Provides: {"impl_24"}
// Dependencies: {}
impl Timestamp { pub fn from_raw_event (raw_event : & RawEvent , start_time : SystemTime) -> Self { debug_assert ! (! raw_event . is_integer ()) ; if raw_event . is_instant () { let t = start_time + Duration :: from_nanos (raw_event . start_value ()) ; Self :: Instant (t) } else { let start = start_time + Duration :: from_nanos (raw_event . start_value ()) ; let end = start_time + Duration :: from_nanos (raw_event . end_value ()) ; Timestamp :: Interval { start , end } } } pub fn contains (& self , t : SystemTime) -> bool { match * self { Timestamp :: Interval { start , end } => t >= start && t < end , Timestamp :: Instant (_) => false , } } pub fn is_instant (& self) -> bool { matches ! (self , & Timestamp :: Instant (_)) } pub fn start (& self) -> SystemTime { match * self { Timestamp :: Interval { start , .. } => start , Timestamp :: Instant (t) => t , } } pub fn end (& self) -> SystemTime { match * self { Timestamp :: Interval { end , .. } => end , Timestamp :: Instant (t) => t , } } pub fn duration (& self) -> Option < Duration > { if let Timestamp :: Interval { start , end } = * self { end . duration_since (start) . ok () } else { None } } }
};
}
