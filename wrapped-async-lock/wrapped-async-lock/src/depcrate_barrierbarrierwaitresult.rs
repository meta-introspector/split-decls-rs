// Generated macro for BarrierWaitResult (struct)
macro_rules! Depcrate_barrierBarrierWaitResult {
() => {
// Module: crate::barrier
// Provides: {"BarrierWaitResult"}
// Dependencies: {}
# [doc = " Returned by [`Barrier::wait()`] when all tasks have called it."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " use async_lock::Barrier;"] # [doc = ""] # [doc = " let barrier = Barrier::new(1);"] # [doc = " let barrier_wait_result = barrier.wait().await;"] # [doc = " # });"] # [doc = " ```"] # [derive (Debug , Clone)] pub struct BarrierWaitResult { is_leader : bool , }
};
}
