// Generated macro for exit (function)
macro_rules! Depcrate_syscalls_tasksexit {
() => {
// Module: crate::syscalls::tasks
// Provides: {"exit"}
// Dependencies: {}
fn exit (arg : i32) -> ! { debug ! ("Exit program with error code {arg}!") ; super :: shutdown (arg) }
};
}
