// Generated macro for ReadyToRunQueue (struct)
macro_rules! Depcrate_stream_futures_unordered_ready_to_run_queueReadyToRunQueue {
() => {
// Module: crate::stream::futures_unordered::ready_to_run_queue
// Provides: {"ReadyToRunQueue"}
// Dependencies: {}
pub (super) struct ReadyToRunQueue < Fut > { pub (super) waker : AtomicWaker , pub (super) head : AtomicPtr < Task < Fut > > , pub (super) tail : UnsafeCell < * const Task < Fut > > , pub (super) stub : Arc < Task < Fut > > , }
};
}
