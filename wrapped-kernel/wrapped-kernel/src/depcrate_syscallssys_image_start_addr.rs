// Generated macro for sys_image_start_addr (function)
macro_rules! Depcrate_syscallssys_image_start_addr {
() => {
// Module: crate::syscalls
// Provides: {"sys_image_start_addr"}
// Dependencies: {}
# [hermit_macro :: system] # [unsafe (no_mangle)] pub extern "C" fn sys_image_start_addr () -> usize { crate :: mm :: kernel_start_address () . as_usize () }
};
}
