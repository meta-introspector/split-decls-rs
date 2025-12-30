// Generated macro for RecursiveMutexState (struct)
macro_rules! Depcrate_synch_recmutexRecursiveMutexState {
() => {
// Module: crate::synch::recmutex
// Provides: {"RecursiveMutexState"}
// Dependencies: {}
struct RecursiveMutexState { current_tid : Option < TaskId > , count : usize , queue : TaskHandlePriorityQueue , }
};
}
