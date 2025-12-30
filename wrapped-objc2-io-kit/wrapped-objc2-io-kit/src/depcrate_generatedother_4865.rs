// Generated macro for other_4865 (other)
macro_rules! Depcrate_generatedother_4865 {
() => {
// Module: crate::generated
// Provides: {"other_4865"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Returns the first parent of a registry entry in a plane."] # [doc = ""] # [doc = " This function will return the parent to which the registry entry was first attached in a plane."] # [doc = ""] # [doc = " Parameter `entry`: The registry entry whose parent to look up."] # [doc = ""] # [doc = " Parameter `plane`: The name of an existing registry plane. Plane names are defined in IOKitKeys.h, eg. kIOServicePlane."] # [doc = ""] # [doc = " Parameter `parent`: The first parent of the registry entry, on success. The parent must be released by the caller."] # [doc = ""] # [doc = " Returns: A kern_return_t error code."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `plane` must be a valid pointer."] # [doc = " - `parent` must be a valid pointer."] # [cfg (feature = "libc")] pub fn IORegistryEntryGetParentEntry (entry : io_registry_entry_t , plane : * mut io_name_t , parent : * mut io_registry_entry_t ,) -> libc :: kern_return_t ; }
};
}
