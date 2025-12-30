// Generated macro for KILL_SIGNALS (const)
macro_rules! Depcrate_signal_handlerKILL_SIGNALS {
() => {
// Module: crate::signal_handler
// Provides: {"KILL_SIGNALS"}
// Dependencies: {}
# [doc = " Signals that represent that we have a bug, and our prompt termination has"] # [doc = " been ordered."] # [rustfmt :: skip] const KILL_SIGNALS : [(libc :: c_int , & str) ; 3] = [(libc :: SIGILL , "SIGILL") , (libc :: SIGBUS , "SIGBUS") , (libc :: SIGSEGV , "SIGSEGV")] ;
};
}
