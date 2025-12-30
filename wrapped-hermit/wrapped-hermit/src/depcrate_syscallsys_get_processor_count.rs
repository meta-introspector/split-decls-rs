// Generated macro for sys_get_processor_count (function)
macro_rules! Depcrate_syscallsys_get_processor_count {
() => {
// Module: crate::syscall
// Provides: {"sys_get_processor_count"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn sys_get_processor_count () -> usize { syscall ! (SyscallNo :: GetProcessorCount) . try_into () . unwrap () }
};
}
