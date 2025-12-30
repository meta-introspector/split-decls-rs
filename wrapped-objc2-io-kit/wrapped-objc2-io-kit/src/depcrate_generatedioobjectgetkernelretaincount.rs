// Generated macro for IOObjectGetKernelRetainCount (function)
macro_rules! Depcrate_generatedIOObjectGetKernelRetainCount {
() => {
// Module: crate::generated
// Provides: {"IOObjectGetKernelRetainCount"}
// Dependencies: {}
# [doc = " Returns kernel retain count of an IOKit object."] # [doc = ""] # [doc = " This function may be used in diagnostics to determine the current retain count of the kernel object at the kernel level."] # [doc = ""] # [doc = " Parameter `object`: An IOKit object."] # [doc = ""] # [doc = " Returns: If the object handle is valid, the kernel objects retain count is returned, otherwise zero is returned."] # [cfg (feature = "libc")] # [inline] pub extern "C-unwind" fn IOObjectGetKernelRetainCount (object : io_object_t) -> u32 { extern "C-unwind" { fn IOObjectGetKernelRetainCount (object : io_object_t) -> u32 ; } unsafe { IOObjectGetKernelRetainCount (object) } }
};
}
