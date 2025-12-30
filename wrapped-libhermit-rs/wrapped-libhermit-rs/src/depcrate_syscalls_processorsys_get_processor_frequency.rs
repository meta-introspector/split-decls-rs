// Generated macro for sys_get_processor_frequency (function)
macro_rules! Depcrate_syscalls_processorsys_get_processor_frequency {
() => {
// Module: crate::syscalls::processor
// Provides: {"sys_get_processor_frequency"}
// Dependencies: {}
# [doc = " Returns the processor frequency in MHz."] # [hermit_macro :: system] # [unsafe (no_mangle)] pub extern "C" fn sys_get_processor_frequency () -> u16 { crate :: arch :: processor :: get_frequency () }
};
}
