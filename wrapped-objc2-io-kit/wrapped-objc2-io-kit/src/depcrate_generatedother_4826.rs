// Generated macro for other_4826 (other)
macro_rules! Depcrate_generatedother_4826 {
() => {
// Module: crate::generated
// Provides: {"other_4826"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Set a CF container based property on a connection."] # [doc = ""] # [doc = " This is a generic method to pass a CF property to the connection. The property is interpreted by the family and commonly represent configuration settings, but may be interpreted as anything."] # [doc = ""] # [doc = " Parameter `connect`: The connect handle created by IOServiceOpen."] # [doc = ""] # [doc = " Parameter `propertyName`: The name of the property as a CFString."] # [doc = ""] # [doc = " Parameter `property`: A CF container - should consist of objects which are understood by IOKit - these are currently : CFDictionary, CFArray, CFSet, CFString, CFData, CFNumber, CFBoolean, and are passed in the kernel as the corresponding OSDictionary etc. objects."] # [doc = ""] # [doc = " Returns: A kern_return_t error code returned by the object."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `property_name` might not allow `None`."] # [doc = " - `property` should be of the correct type."] # [doc = " - `property` might not allow `None`."] # [cfg (feature = "libc")] pub fn IOConnectSetCFProperty (connect : io_connect_t , property_name : Option < & CFString > , property : Option < & CFType > ,) -> libc :: kern_return_t ; }
};
}
