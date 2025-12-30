// Generated macro for impl_461 (impl)
macro_rules! Depcrate_common_timerimpl_461 {
() => {
// Module: crate::common::timer
// Provides: {"impl_461"}
// Dependencies: {}
impl hyper :: rt :: Timer for Timer { fn sleep (& self , duration : Duration) -> Pin < Box < dyn Sleep > > { self . 0 . sleep (duration) } fn sleep_until (& self , deadline : Instant) -> Pin < Box < dyn Sleep > > { self . 0 . sleep_until (deadline) } fn now (& self) -> Instant { self . 0 . now () } }
};
}
