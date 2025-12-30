// Generated macro for other_4860 (other)
macro_rules! Depcrate_generatedother_4860 {
() => {
// Module: crate::generated
// Provides: {"other_4860"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Set CF container based properties in a registry entry."] # [doc = ""] # [doc = " This is a generic method to pass a CF container of properties to an object in the registry. Setting properties in a registry entry is not generally supported, it is more common to support IOConnectSetCFProperties for connection based property setting. The properties are interpreted by the object."] # [doc = ""] # [doc = " Parameter `entry`: The registry entry whose properties to set."] # [doc = ""] # [doc = " Parameter `properties`: A CF container - commonly a CFDictionary but this is not enforced. The container should consist of objects which are understood by IOKit - these are currently : CFDictionary, CFArray, CFSet, CFString, CFData, CFNumber, CFBoolean, and are passed in the kernel as the corresponding OSDictionary etc. objects."] # [doc = ""] # [doc = " Returns: A kern_return_t error code returned by the object."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `properties` should be of the correct type."] # [doc = " - `properties` might not allow `None`."] # [cfg (feature = "libc")] pub fn IORegistryEntrySetCFProperties (entry : io_registry_entry_t , properties : Option < & CFType > ,) -> libc :: kern_return_t ; }
};
}
