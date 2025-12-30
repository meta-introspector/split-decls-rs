// Generated macro for impl_29 (impl)
macro_rules! Depcrate_clockimpl_29 {
() => {
// Module: crate::clock
// Provides: {"impl_29"}
// Dependencies: {}
impl GetTime for ThreadCpuTime { fn time () -> Duration { time (libc :: CLOCK_THREAD_CPUTIME_ID) } }
};
}
