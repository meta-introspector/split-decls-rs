// Generated macro for IORegistryEntryCopyPath (function)
macro_rules! Depcrate_generatedIORegistryEntryCopyPath {
() => {
// Module: crate::generated
// Provides: {"IORegistryEntryCopyPath"}
// Dependencies: {}
# [doc = " Create a path for a registry entry."] # [doc = ""] # [doc = " The path for a registry entry is returned as a CFString The path describes the entry's attachment in a particular plane, which must be specified. The path begins with the plane name followed by a colon, and then followed by '/' separated path components for each of the entries between the root and the registry entry. An alias may also exist for the entry, and will be returned if available."] # [doc = ""] # [doc = " Parameter `entry`: The registry entry handle whose path to look up."] # [doc = ""] # [doc = " Parameter `plane`: The name of an existing registry plane. Plane names are defined in IOKitKeys.h, eg. kIOServicePlane."] # [doc = ""] # [doc = " Returns: An instance of CFString on success, to be released by the caller. IORegistryEntryCopyPath will fail if the entry is not attached in the plane."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `plane` must be a valid pointer."] # [cfg (feature = "libc")] # [inline] pub unsafe extern "C-unwind" fn IORegistryEntryCopyPath (entry : io_registry_entry_t , plane : * mut io_name_t ,) -> Option < CFRetained < CFString > > { extern "C-unwind" { fn IORegistryEntryCopyPath (entry : io_registry_entry_t , plane : * mut io_name_t ,) -> Option < NonNull < CFString > > ; } let ret = unsafe { IORegistryEntryCopyPath (entry , plane) } ; ret . map (| ret | unsafe { CFRetained :: from_raw (ret) }) }
};
}
