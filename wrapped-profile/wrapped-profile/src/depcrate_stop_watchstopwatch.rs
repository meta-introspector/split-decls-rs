// Generated macro for StopWatch (struct)
macro_rules! Depcrate_stop_watchStopWatch {
() => {
// Module: crate::stop_watch
// Provides: {"StopWatch"}
// Dependencies: {}
pub struct StopWatch { time : Instant , # [cfg (all (target_os = "linux" , not (target_env = "ohos")))] counter : Option < perf_event :: Counter > , memory : MemoryUsage , }
};
}
