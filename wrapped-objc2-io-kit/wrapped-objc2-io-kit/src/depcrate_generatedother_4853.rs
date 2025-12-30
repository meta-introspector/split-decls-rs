// Generated macro for other_4853 (other)
macro_rules! Depcrate_generatedother_4853 {
() => {
// Module: crate::generated
// Provides: {"other_4853"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Create a path for a registry entry."] # [doc = ""] # [doc = " The path for a registry entry is copied to the caller's buffer. The path describes the entry's attachment in a particular plane, which must be specified. The path begins with the plane name followed by a colon, and then followed by '/' separated path components for each of the entries between the root and the registry entry. An alias may also exist for the entry, and will be returned if available."] # [doc = ""] # [doc = " Parameter `entry`: The registry entry handle whose path to look up."] # [doc = ""] # [doc = " Parameter `plane`: The name of an existing registry plane. Plane names are defined in IOKitKeys.h, eg. kIOServicePlane."] # [doc = ""] # [doc = " Parameter `path`: A char buffer allocated by the caller."] # [doc = ""] # [doc = " Returns: IORegistryEntryGetPath will fail if the entry is not attached in the plane, or if the buffer is not large enough to contain the path."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `plane` must be a valid pointer."] # [doc = " - `path` must be a valid pointer."] # [cfg (feature = "libc")] pub fn IORegistryEntryGetPath (entry : io_registry_entry_t , plane : * mut io_name_t , path : * mut io_string_t ,) -> libc :: kern_return_t ; }
};
}
