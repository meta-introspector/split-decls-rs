// Generated macro for ThreadJoinStatus (enum)
macro_rules! Depcrate_concurrency_threadThreadJoinStatus {
() => {
// Module: crate::concurrency::thread
// Provides: {"ThreadJoinStatus"}
// Dependencies: {}
# [doc = " The join status of a thread."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] enum ThreadJoinStatus { # [doc = " The thread can be joined."] Joinable , # [doc = " A thread is detached if its join handle was destroyed and no other"] # [doc = " thread can join it."] Detached , # [doc = " The thread was already joined by some thread and cannot be joined again."] Joined , }
};
}
