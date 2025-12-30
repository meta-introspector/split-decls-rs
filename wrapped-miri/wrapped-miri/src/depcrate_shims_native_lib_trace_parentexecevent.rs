// Generated macro for ExecEvent (enum)
macro_rules! Depcrate_shims_native_lib_trace_parentExecEvent {
() => {
// Module: crate::shims::native_lib::trace::parent
// Provides: {"ExecEvent"}
// Dependencies: {}
# [doc = " A unified event representing something happening on the child process. Wraps"] # [doc = " `nix`'s `WaitStatus` and our custom signals so it can all be done with one"] # [doc = " `match` statement."] pub enum ExecEvent { # [doc = " Child process requests that we begin monitoring it."] Start (StartFfiInfo) , # [doc = " Child requests that we stop monitoring and pass over the events we"] # [doc = " detected."] End , # [doc = " The child process with the specified pid was stopped by the given signal."] Status (unistd :: Pid , signal :: Signal) , # [doc = " The child process with the specified pid entered or existed a syscall."] Syscall (unistd :: Pid) , # [doc = " A child process exited or was killed; if we have a return code, it is"] # [doc = " specified."] Died (Option < i32 >) , }
};
}
