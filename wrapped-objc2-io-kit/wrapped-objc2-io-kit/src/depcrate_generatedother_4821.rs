// Generated macro for other_4821 (other)
macro_rules! Depcrate_generatedother_4821 {
() => {
// Module: crate::generated
// Provides: {"other_4821"}
// Dependencies: {}
extern "C-unwind" { # [doc = " # Safety"] # [doc = ""] # [doc = " - `at_address` must be a valid pointer."] # [doc = " - `of_size` must be a valid pointer."] # [cfg (feature = "libc")] pub fn IOConnectMapMemory (connect : io_connect_t , memory_type : u32 , into_task : task_port_t , at_address : * mut libc :: mach_vm_address_t , of_size : * mut libc :: mach_vm_size_t , options : IOOptionBits ,) -> libc :: kern_return_t ; }
};
}
