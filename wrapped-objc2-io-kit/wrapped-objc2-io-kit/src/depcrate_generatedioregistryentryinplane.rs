// Generated macro for IORegistryEntryInPlane (function)
macro_rules! Depcrate_generatedIORegistryEntryInPlane {
() => {
// Module: crate::generated
// Provides: {"IORegistryEntryInPlane"}
// Dependencies: {}
# [doc = " Determines if the registry entry is attached in a plane."] # [doc = ""] # [doc = " This method determines if the entry is attached in a plane to any other entry."] # [doc = ""] # [doc = " Parameter `entry`: The registry entry."] # [doc = ""] # [doc = " Parameter `plane`: The name of an existing registry plane. Plane names are defined in IOKitKeys.h, eg. kIOServicePlane."] # [doc = ""] # [doc = " Returns: If the entry has a parent in the plane, true is returned, otherwise false is returned."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `plane` must be a valid pointer."] # [cfg (feature = "libc")] # [inline] pub unsafe extern "C-unwind" fn IORegistryEntryInPlane (entry : io_registry_entry_t , plane : * mut io_name_t ,) -> bool { extern "C-unwind" { fn IORegistryEntryInPlane (entry : io_registry_entry_t , plane : * mut io_name_t ,) -> libc :: boolean_t ; } let ret = unsafe { IORegistryEntryInPlane (entry , plane) } ; ret != 0 }
};
}
