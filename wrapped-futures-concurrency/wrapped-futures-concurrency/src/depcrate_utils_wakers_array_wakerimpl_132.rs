// Generated macro for impl_132 (impl)
macro_rules! Depcrate_utils_wakers_array_wakerimpl_132 {
() => {
// Module: crate::utils::wakers::array::waker
// Provides: {"impl_132"}
// Dependencies: {}
impl < const N : usize > InlineWakerArray < N > { # [doc = " Create a new instance of `InlineWaker`."] pub (crate) fn new (id : usize , readiness : Arc < Mutex < ReadinessArray < N > > >) -> Self { Self { id , readiness } } }
};
}
