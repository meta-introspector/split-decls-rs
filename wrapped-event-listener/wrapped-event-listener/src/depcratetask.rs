// Generated macro for Task (enum)
macro_rules! DepcrateTask {
() => {
// Module: crate
// Provides: {"Task"}
// Dependencies: {}
# [doc = " A task that can be woken up."] # [derive (Debug , Clone)] enum Task { # [doc = " A waker that wakes up a future."] Waker (Waker) , # [doc = " An unparker that wakes up a thread."] # [cfg (all (feature = "std" , not (target_family = "wasm")))] Unparker (Unparker) , }
};
}
