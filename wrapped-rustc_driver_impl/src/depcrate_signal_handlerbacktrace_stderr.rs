// Generated macro for backtrace_stderr (function)
macro_rules! Depcrate_signal_handlerbacktrace_stderr {
() => {
// Module: crate::signal_handler
// Provides: {"backtrace_stderr"}
// Dependencies: {}
fn backtrace_stderr (buffer : & [* mut libc :: c_void]) { let size = buffer . len () . try_into () . unwrap_or_default () ; unsafe { backtrace_symbols_fd (buffer . as_ptr () , size , libc :: STDERR_FILENO) } ; }
};
}
