// Generated macro for other_4863 (other)
macro_rules! Depcrate_generatedother_4863 {
() => {
// Module: crate::generated
// Provides: {"other_4863"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Returns the first child of a registry entry in a plane."] # [doc = ""] # [doc = " This function will return the child which first attached to a registry entry in a plane."] # [doc = ""] # [doc = " Parameter `entry`: The registry entry whose child to look up."] # [doc = ""] # [doc = " Parameter `plane`: The name of an existing registry plane. Plane names are defined in IOKitKeys.h, eg. kIOServicePlane."] # [doc = ""] # [doc = " Parameter `child`: The first child of the registry entry, on success. The child must be released by the caller."] # [doc = ""] # [doc = " Returns: A kern_return_t error code."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `plane` must be a valid pointer."] # [doc = " - `child` must be a valid pointer."] # [cfg (feature = "libc")] pub fn IORegistryEntryGetChildEntry (entry : io_registry_entry_t , plane : * mut io_name_t , child : * mut io_registry_entry_t ,) -> libc :: kern_return_t ; }
};
}
