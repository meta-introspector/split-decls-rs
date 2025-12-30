// Generated macro for Semaphore (struct)
macro_rules! Depcrate_semaphoreSemaphore {
() => {
// Module: crate::semaphore
// Provides: {"Semaphore"}
// Dependencies: {}
# [doc = " A counter for limiting the number of concurrent operations."] # [derive (Debug)] pub struct Semaphore { count : AtomicUsize , event : Event , }
};
}
