// Generated macro for other_4852 (other)
macro_rules! Depcrate_generatedother_4852 {
() => {
// Module: crate::generated
// Provides: {"other_4852"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Returns a C-string location assigned to a registry entry, in a specified plane."] # [doc = ""] # [doc = " Registry entries can given a location string in a particular plane, or globally. If the entry has had a location set in the specified plane that location string will be returned, otherwise the global location string is returned. If no global location string has been set, an error is returned."] # [doc = ""] # [doc = " Parameter `entry`: The registry entry handle whose name to look up."] # [doc = ""] # [doc = " Parameter `plane`: The name of an existing registry plane. Plane names are defined in IOKitKeys.h, eg. kIOServicePlane."] # [doc = ""] # [doc = " Parameter `location`: The caller's buffer to receive the location string."] # [doc = ""] # [doc = " Returns: A kern_return_t error code."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `plane` must be a valid pointer."] # [doc = " - `location` must be a valid pointer."] # [cfg (feature = "libc")] pub fn IORegistryEntryGetLocationInPlane (entry : io_registry_entry_t , plane : * mut io_name_t , location : * mut io_name_t ,) -> libc :: kern_return_t ; }
};
}
