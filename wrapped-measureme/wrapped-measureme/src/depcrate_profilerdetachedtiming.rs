// Generated macro for DetachedTiming (struct)
macro_rules! Depcrate_profilerDetachedTiming {
() => {
// Module: crate::profiler
// Provides: {"DetachedTiming"}
// Dependencies: {}
# [doc = " Created by `Profiler::start_recording_interval_event_detached`."] # [doc = " Must be passed to `finish_recording_interval_event` to record an"] # [doc = " \"end\" event."] # [must_use] pub struct DetachedTiming { event_id : EventId , event_kind : StringId , thread_id : u32 , start_count : u64 , }
};
}
