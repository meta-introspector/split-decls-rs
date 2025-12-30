// Generated macro for sys_get_processor_count (function)
macro_rules! Depcrate_syscalls_processorsys_get_processor_count {
() => {
// Module: crate::syscalls::processor
// Provides: {"sys_get_processor_count"}
// Dependencies: {}
# [doc = " Returns the number of processors currently online."] # [hermit_macro :: system] # [unsafe (no_mangle)] pub extern "C" fn sys_get_processor_count () -> usize { get_processor_count () . try_into () . unwrap () }
};
}
