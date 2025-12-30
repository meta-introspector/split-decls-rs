// Generated macro for IOObjectGetRetainCount (function)
macro_rules! Depcrate_generatedIOObjectGetRetainCount {
() => {
// Module: crate::generated
// Provides: {"IOObjectGetRetainCount"}
// Dependencies: {}
# [doc = " Returns kernel retain count of an IOKit object. Identical to IOObjectGetKernelRetainCount() but available prior to Mac OS 10.6."] # [doc = ""] # [doc = " This function may be used in diagnostics to determine the current retain count of the kernel object at the kernel level."] # [doc = ""] # [doc = " Parameter `object`: An IOKit object."] # [doc = ""] # [doc = " Returns: If the object handle is valid, the kernel objects retain count is returned, otherwise zero is returned."] # [cfg (feature = "libc")] # [inline] pub extern "C-unwind" fn IOObjectGetRetainCount (object : io_object_t) -> u32 { extern "C-unwind" { fn IOObjectGetRetainCount (object : io_object_t) -> u32 ; } unsafe { IOObjectGetRetainCount (object) } }
};
}
