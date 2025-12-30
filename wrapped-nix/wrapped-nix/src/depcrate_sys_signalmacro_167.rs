// Generated macro for macro_167 (macro)
macro_rules! Depcrate_sys_signalmacro_167 {
() => {
// Module: crate::sys::signal
// Provides: {"macro_167"}
// Dependencies: {}
# [cfg (feature = "signal")] libc_bitflags ! { # [doc = " Controls the behavior of a [`SigAction`]"] # [cfg_attr (docsrs , doc (cfg (feature = "signal")))] pub struct SaFlags : SaFlags_t { # [doc = " When catching a [`Signal::SIGCHLD`] signal, the signal will be"] # [doc = " generated only when a child process exits, not when a child process"] # [doc = " stops."] SA_NOCLDSTOP ; # [doc = " When catching a [`Signal::SIGCHLD`] signal, the system will not"] # [doc = " create zombie processes when children of the calling process exit."] # [cfg (not (target_os = "hurd"))] SA_NOCLDWAIT ; # [doc = " Further occurrences of the delivered signal are not masked during"] # [doc = " the execution of the handler."] SA_NODEFER ; # [doc = " The system will deliver the signal to the process on a signal stack,"] # [doc = " specified by each thread with sigaltstack(2)."] SA_ONSTACK ; # [doc = " The handler is reset back to the default at the moment the signal is"] # [doc = " delivered."] SA_RESETHAND ; # [doc = " Requests that certain system calls restart if interrupted by this"] # [doc = " signal.  See the man page for complete details."] SA_RESTART ; # [doc = " This flag is controlled internally by Nix."] SA_SIGINFO ; } }
};
}
