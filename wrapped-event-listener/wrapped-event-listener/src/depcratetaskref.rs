// Generated macro for TaskRef (enum)
macro_rules! DepcrateTaskRef {
() => {
// Module: crate
// Provides: {"TaskRef"}
// Dependencies: {}
# [doc = " A reference to a task."] # [derive (Clone , Copy)] enum TaskRef < 'a > { # [doc = " A waker that wakes up a future."] Waker (& 'a Waker) , # [doc = " An unparker that wakes up a thread."] # [cfg (all (feature = "std" , not (target_family = "wasm")))] Unparker (& 'a Unparker) , }
};
}
