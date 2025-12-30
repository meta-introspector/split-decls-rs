// Generated macro for WakerQueue (struct)
macro_rules! Depcrate_waker_queueWakerQueue {
() => {
// Module: crate::waker_queue
// Provides: {"WakerQueue"}
// Dependencies: {}
# [doc = " `mio::Waker` with a queue for waking up the `Accept`'s `Poll` and contains the `WakerInterest`"] # [doc = " the `Poll` would want to look into."] pub (crate) struct WakerQueue (Arc < (Waker , Mutex < VecDeque < WakerInterest > >) >) ;
};
}
