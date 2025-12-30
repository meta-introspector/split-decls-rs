// Generated macro for mem_blocks_t (type)
macro_rules! Depcrate_sys_sysinfomem_blocks_t {
() => {
// Module: crate::sys::sysinfo
// Provides: {"mem_blocks_t"}
// Dependencies: {}
# [cfg (not (all (target_arch = "x86_64" , target_pointer_width = "32")))] type mem_blocks_t = libc :: c_ulong ;
};
}
