// Generated macro for IOObjectGetUserRetainCount (function)
macro_rules! Depcrate_generatedIOObjectGetUserRetainCount {
() => {
// Module: crate::generated
// Provides: {"IOObjectGetUserRetainCount"}
// Dependencies: {}
# [doc = " Returns the retain count for the current process of an IOKit object."] # [doc = ""] # [doc = " This function may be used in diagnostics to determine the current retain count for the calling process of the kernel object."] # [doc = ""] # [doc = " Parameter `object`: An IOKit object."] # [doc = ""] # [doc = " Returns: If the object handle is valid, the objects user retain count is returned, otherwise zero is returned."] # [cfg (feature = "libc")] # [inline] pub extern "C-unwind" fn IOObjectGetUserRetainCount (object : io_object_t) -> u32 { extern "C-unwind" { fn IOObjectGetUserRetainCount (object : io_object_t) -> u32 ; } unsafe { IOObjectGetUserRetainCount (object) } }
};
}
