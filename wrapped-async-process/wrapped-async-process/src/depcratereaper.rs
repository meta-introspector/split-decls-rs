// Generated macro for Reaper (struct)
macro_rules! DepcrateReaper {
() => {
// Module: crate
// Provides: {"Reaper"}
// Dependencies: {}
# [doc = " The zombie process reaper."] # [doc = ""] # [doc = " This structure reaps zombie processes and emits the `SIGCHLD` signal."] struct Reaper { # [doc = " Underlying system reaper."] sys : reaper :: Reaper , # [doc = " The number of tasks polling the SIGCHLD event."] # [doc = ""] # [doc = " If this is zero, the `async-process` thread must be spawned."] drivers : AtomicUsize , # [doc = " Number of live `Child` instances currently running."] # [doc = ""] # [doc = " This is used to prevent the reaper thread from being spawned right as the program closes,"] # [doc = " when the reaper thread isn't needed. This represents the number of active processes."] child_count : AtomicUsize , }
};
}
