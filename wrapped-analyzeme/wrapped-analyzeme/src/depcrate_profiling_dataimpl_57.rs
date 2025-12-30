// Generated macro for impl_57 (impl)
macro_rules! Depcrate_profiling_dataimpl_57 {
() => {
// Module: crate::profiling_data
// Provides: {"impl_57"}
// Dependencies: {}
impl < 'a > ProfilerEventIterator < 'a > { pub fn new (data : & 'a ProfilingData) -> ProfilerEventIterator < 'a > { ProfilerEventIterator { data , forward_event_idx : 0 , backward_event_idx : data . num_events () , } } }
};
}
