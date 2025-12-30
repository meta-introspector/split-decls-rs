// Generated macro for other_4825 (other)
macro_rules! Depcrate_generatedother_4825 {
() => {
// Module: crate::generated
// Provides: {"other_4825"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Set CF container based properties on a connection."] # [doc = ""] # [doc = " This is a generic method to pass a CF container of properties to the connection. The properties are interpreted by the family and commonly represent configuration settings, but may be interpreted as anything."] # [doc = ""] # [doc = " Parameter `connect`: The connect handle created by IOServiceOpen."] # [doc = ""] # [doc = " Parameter `properties`: A CF container - commonly a CFDictionary but this is not enforced. The container should consist of objects which are understood by IOKit - these are currently : CFDictionary, CFArray, CFSet, CFString, CFData, CFNumber, CFBoolean, and are passed in the kernel as the corresponding OSDictionary etc. objects."] # [doc = ""] # [doc = " Returns: A kern_return_t error code returned by the family."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `properties` should be of the correct type."] # [doc = " - `properties` might not allow `None`."] # [cfg (feature = "libc")] pub fn IOConnectSetCFProperties (connect : io_connect_t , properties : Option < & CFType > ,) -> libc :: kern_return_t ; }
};
}
