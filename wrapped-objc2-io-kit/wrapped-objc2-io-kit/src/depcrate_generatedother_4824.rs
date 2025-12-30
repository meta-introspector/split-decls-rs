// Generated macro for other_4824 (other)
macro_rules! Depcrate_generatedother_4824 {
() => {
// Module: crate::generated
// Provides: {"other_4824"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Remove a mapping made with IOConnectMapMemory64."] # [doc = ""] # [doc = " This is a generic method to remove a mapping in the callers task."] # [doc = ""] # [doc = " Parameter `connect`: The connect handle created by IOServiceOpen."] # [doc = ""] # [doc = " Parameter `memoryType`: The memory type originally requested in IOConnectMapMemory."] # [doc = ""] # [doc = " Parameter `fromTask`: The task port for the task in which to remove the mapping. This may be different to the task which the opened the connection."] # [doc = ""] # [doc = " Parameter `atAddress`: The address of the mapping to be removed."] # [doc = ""] # [doc = " Returns: A kern_return_t error code."] # [cfg (feature = "libc")] pub fn IOConnectUnmapMemory64 (connect : io_connect_t , memory_type : u32 , from_task : task_port_t , at_address : libc :: mach_vm_address_t ,) -> libc :: kern_return_t ; }
};
}
