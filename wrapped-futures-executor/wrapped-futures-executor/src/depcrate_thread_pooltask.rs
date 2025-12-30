// Generated macro for Task (struct)
macro_rules! Depcrate_thread_poolTask {
() => {
// Module: crate::thread_pool
// Provides: {"Task"}
// Dependencies: {}
# [doc = " A task responsible for polling a future to completion."] struct Task { future : FutureObj < 'static , () > , exec : ThreadPool , wake_handle : Arc < WakeHandle > , }
};
}
