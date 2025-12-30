// Generated macro for WakeHandle (struct)
macro_rules! Depcrate_thread_poolWakeHandle {
() => {
// Module: crate::thread_pool
// Provides: {"WakeHandle"}
// Dependencies: {}
struct WakeHandle { mutex : UnparkMutex < Task > , exec : ThreadPool , }
};
}
