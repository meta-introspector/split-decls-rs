// Generated macro for impl_20 (impl)
macro_rules! Depcrate_barrierimpl_20 {
() => {
// Module: crate::barrier
// Provides: {"impl_20"}
// Dependencies: {}
impl BarrierWaitResult { # [doc = " Returns `true` if this task was the last to call to [`Barrier::wait()`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " use async_lock::Barrier;"] # [doc = " use futures_lite::future;"] # [doc = ""] # [doc = " let barrier = Barrier::new(2);"] # [doc = " let (a, b) = future::zip(barrier.wait(), barrier.wait()).await;"] # [doc = " assert_eq!(a.is_leader(), false);"] # [doc = " assert_eq!(b.is_leader(), true);"] # [doc = " # });"] # [doc = " ```"] pub fn is_leader (& self) -> bool { self . is_leader } }
};
}
