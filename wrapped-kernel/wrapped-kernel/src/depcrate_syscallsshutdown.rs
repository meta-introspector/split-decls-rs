// Generated macro for shutdown (function)
macro_rules! Depcrate_syscallsshutdown {
() => {
// Module: crate::syscalls
// Provides: {"shutdown"}
// Dependencies: {}
pub (crate) fn shutdown (arg : i32) -> ! { crate :: arch :: kernel :: print_statistics () ; SYS . shutdown (arg) }
};
}
