// Generated macro for impl_60 (impl)
macro_rules! Depcrate_profilerimpl_60 {
() => {
// Module: crate::profiler
// Provides: {"impl_60"}
// Dependencies: {}
impl < 'a > TimingGuard < 'a > { # [doc = " This method set a new `event_id` right before actually recording the"] # [doc = " event."] # [inline] pub fn finish_with_override_event_id (mut self , event_id : EventId) { self . event_id = event_id ; drop (self) } }
};
}
