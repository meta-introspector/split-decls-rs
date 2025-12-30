// Generated macro for impl_28 (impl)
macro_rules! Depcrate_clockimpl_28 {
() => {
// Module: crate::clock
// Provides: {"impl_28"}
// Dependencies: {}
impl GetTime for Monotonic { fn time () -> Duration { time (libc :: CLOCK_MONOTONIC) } }
};
}
