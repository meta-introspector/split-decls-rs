// Generated macro for Counter (struct)
macro_rules! Depcrate_workerCounter {
() => {
// Module: crate::worker
// Provides: {"Counter"}
// Dependencies: {}
# [doc = " counter: Arc<AtomicUsize> field is owned by `Accept` thread and `ServerWorker` thread."] # [doc = ""] # [doc = " `Accept` would increment the counter and `ServerWorker` would decrement it."] # [doc = ""] # [doc = " # Atomic Ordering:"] # [doc = ""] # [doc = " `Accept` always look into it's cached `Availability` field for `ServerWorker` state."] # [doc = " It lazily increment counter after successful dispatching new work to `ServerWorker`."] # [doc = " On reaching counter limit `Accept` update it's cached `Availability` and mark worker as"] # [doc = " unable to accept any work."] # [doc = ""] # [doc = " `ServerWorker` always decrement the counter when every work received from `Accept` is done."] # [doc = " On reaching counter limit worker would use `mio::Waker` and `WakerQueue` to wake up `Accept`"] # [doc = " and notify it to update cached `Availability` again to mark worker as able to accept work again."] # [doc = ""] # [doc = " Hence, a wake up would only happen after `Accept` increment it to limit."] # [doc = " And a decrement to limit always wake up `Accept`."] # [derive (Clone)] pub (crate) struct Counter { counter : Arc < AtomicUsize > , limit : usize , }
};
}
