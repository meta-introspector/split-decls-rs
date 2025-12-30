// Generated macro for start (function)
macro_rules! Depcrate_google_cpu_profilerstart {
() => {
// Module: crate::google_cpu_profiler
// Provides: {"start"}
// Dependencies: {}
pub (crate) fn start (path : & Path) { if ! transition (OFF , PENDING) { panic ! ("profiler already started") ; } let path = CString :: new (path . display () . to_string ()) . unwrap () ; if unsafe { ProfilerStart (path . as_ptr ()) } == 0 { panic ! ("profiler failed to start") } assert ! (transition (PENDING , ON)) ; }
};
}
