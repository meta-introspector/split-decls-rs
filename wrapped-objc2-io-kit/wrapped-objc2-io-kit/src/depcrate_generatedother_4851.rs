// Generated macro for other_4851 (other)
macro_rules! Depcrate_generatedother_4851 {
() => {
// Module: crate::generated
// Provides: {"other_4851"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Returns a C-string name assigned to a registry entry, in a specified plane."] # [doc = ""] # [doc = " Registry entries can be named in a particular plane, or globally. This function returns the entry's name in the specified plane or global name if it has not been named in that plane. The global name defaults to the entry's meta class name if it has not been named."] # [doc = ""] # [doc = " Parameter `entry`: The registry entry handle whose name to look up."] # [doc = ""] # [doc = " Parameter `plane`: The name of an existing registry plane. Plane names are defined in IOKitKeys.h, eg. kIOServicePlane."] # [doc = ""] # [doc = " Parameter `name`: The caller's buffer to receive the name."] # [doc = ""] # [doc = " Returns: A kern_return_t error code."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `plane` must be a valid pointer."] # [doc = " - `name` must be a valid pointer."] # [cfg (feature = "libc")] pub fn IORegistryEntryGetNameInPlane (entry : io_registry_entry_t , plane : * mut io_name_t , name : * mut io_name_t ,) -> libc :: kern_return_t ; }
};
}
