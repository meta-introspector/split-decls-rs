// Generated macro for impl_58 (impl)
macro_rules! Depcrate_profiling_dataimpl_58 {
() => {
// Module: crate::profiling_data
// Provides: {"impl_58"}
// Dependencies: {}
impl < 'a > Iterator for ProfilerEventIterator < 'a > { type Item = LightweightEvent ; fn next (& mut self) -> Option < LightweightEvent > { if self . forward_event_idx == self . backward_event_idx { return None ; } let event = Some (self . data . decode_lightweight_event (self . forward_event_idx)) ; self . forward_event_idx = self . forward_event_idx . checked_add (1) . unwrap () ; event } fn size_hint (& self) -> (usize , Option < usize >) { let items_left = self . backward_event_idx . checked_sub (self . forward_event_idx) . unwrap () ; (items_left , Some (items_left)) } }
};
}
