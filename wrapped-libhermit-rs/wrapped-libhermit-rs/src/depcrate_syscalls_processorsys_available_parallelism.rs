// Generated macro for sys_available_parallelism (function)
macro_rules! Depcrate_syscalls_processorsys_available_parallelism {
() => {
// Module: crate::syscalls::processor
// Provides: {"sys_available_parallelism"}
// Dependencies: {}
# [hermit_macro :: system] # [unsafe (no_mangle)] pub extern "C" fn sys_available_parallelism () -> usize { get_processor_count () . try_into () . unwrap () }
};
}
