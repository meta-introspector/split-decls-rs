// Generated macro for SemaphoreState (struct)
macro_rules! Depcrate_synch_semaphoreSemaphoreState {
() => {
// Module: crate::synch::semaphore
// Provides: {"SemaphoreState"}
// Dependencies: {}
struct SemaphoreState { # [doc = " Resource available count"] count : isize , # [doc = " Priority queue of waiting tasks"] queue : TaskHandlePriorityQueue , }
};
}
