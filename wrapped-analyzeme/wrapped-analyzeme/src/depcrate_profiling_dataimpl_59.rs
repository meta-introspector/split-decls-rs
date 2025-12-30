// Generated macro for impl_59 (impl)
macro_rules! Depcrate_profiling_dataimpl_59 {
() => {
// Module: crate::profiling_data
// Provides: {"impl_59"}
// Dependencies: {}
impl < 'a > DoubleEndedIterator for ProfilerEventIterator < 'a > { fn next_back (& mut self) -> Option < Self :: Item > { if self . forward_event_idx == self . backward_event_idx { return None ; } self . backward_event_idx = self . backward_event_idx . checked_sub (1) . unwrap () ; Some (self . data . decode_lightweight_event (self . backward_event_idx)) } }
};
}
