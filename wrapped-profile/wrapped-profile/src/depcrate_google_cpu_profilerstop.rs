// Generated macro for stop (function)
macro_rules! Depcrate_google_cpu_profilerstop {
() => {
// Module: crate::google_cpu_profiler
// Provides: {"stop"}
// Dependencies: {}
pub (crate) fn stop () { if ! transition (ON , PENDING) { panic ! ("profiler is not started") } unsafe { ProfilerStop () } ; assert ! (transition (PENDING , OFF)) ; }
};
}
