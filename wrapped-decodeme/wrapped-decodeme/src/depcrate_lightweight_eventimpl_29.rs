// Generated macro for impl_29 (impl)
macro_rules! Depcrate_lightweight_eventimpl_29 {
() => {
// Module: crate::lightweight_event
// Provides: {"impl_29"}
// Dependencies: {}
impl LightweightEvent { # [doc = " Returns true if the time interval of `self` completely contains the"] # [doc = " time interval of `other`."] pub fn contains (& self , other : & LightweightEvent) -> bool { self . payload . contains (& other . payload) } pub fn duration (& self) -> Option < Duration > { self . payload . duration () } pub fn start (& self) -> Option < SystemTime > { self . payload . timestamp () . map (| t | t . start ()) } pub fn end (& self) -> Option < SystemTime > { self . payload . timestamp () . map (| t | t . end ()) } pub fn timestamp (& self) -> Option < Timestamp > { self . payload . timestamp () } }
};
}
