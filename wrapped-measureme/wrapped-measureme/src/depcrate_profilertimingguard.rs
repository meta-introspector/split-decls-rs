// Generated macro for TimingGuard (struct)
macro_rules! Depcrate_profilerTimingGuard {
() => {
// Module: crate::profiler
// Provides: {"TimingGuard"}
// Dependencies: {}
# [doc = " When dropped, this `TimingGuard` will record an \"end\" event in the"] # [doc = " `Profiler` it was created by."] # [must_use] pub struct TimingGuard < 'a > { profiler : & 'a Profiler , event_id : EventId , event_kind : StringId , thread_id : u32 , start_count : u64 , }
};
}
