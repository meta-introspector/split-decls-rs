// Generated macro for IOObjectRelease (function)
macro_rules! Depcrate_generatedIOObjectRelease {
() => {
// Module: crate::generated
// Provides: {"IOObjectRelease"}
// Dependencies: {}
# [doc = " Releases an object handle previously returned by IOKitLib."] # [doc = ""] # [doc = " All objects returned by IOKitLib should be released with this function when access to them is no longer needed. Using the object after it has been released may or may not return an error, depending on how many references the task has to the same object in the kernel."] # [doc = ""] # [doc = " Parameter `object`: The IOKit object to release."] # [doc = ""] # [doc = " Returns: A kern_return_t error code."] # [cfg (feature = "libc")] # [inline] pub extern "C-unwind" fn IOObjectRelease (object : io_object_t) -> libc :: kern_return_t { extern "C-unwind" { fn IOObjectRelease (object : io_object_t) -> libc :: kern_return_t ; } unsafe { IOObjectRelease (object) } }
};
}
