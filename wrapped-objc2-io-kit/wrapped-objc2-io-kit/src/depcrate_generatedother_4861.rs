// Generated macro for other_4861 (other)
macro_rules! Depcrate_generatedother_4861 {
() => {
// Module: crate::generated
// Provides: {"other_4861"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Set a CF container based property in a registry entry."] # [doc = ""] # [doc = " This is a generic method to pass a CF container as a property to an object in the registry. Setting properties in a registry entry is not generally supported, it is more common to support IOConnectSetCFProperty for connection based property setting. The property is interpreted by the object."] # [doc = ""] # [doc = " Parameter `entry`: The registry entry whose property to set."] # [doc = ""] # [doc = " Parameter `propertyName`: The name of the property as a CFString."] # [doc = ""] # [doc = " Parameter `property`: A CF container - should consist of objects which are understood by IOKit - these are currently : CFDictionary, CFArray, CFSet, CFString, CFData, CFNumber, CFBoolean, and are passed in the kernel as the corresponding OSDictionary etc. objects."] # [doc = ""] # [doc = " Returns: A kern_return_t error code returned by the object."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `property_name` might not allow `None`."] # [doc = " - `property` should be of the correct type."] # [doc = " - `property` might not allow `None`."] # [cfg (feature = "libc")] pub fn IORegistryEntrySetCFProperty (entry : io_registry_entry_t , property_name : Option < & CFString > , property : Option < & CFType > ,) -> libc :: kern_return_t ; }
};
}
