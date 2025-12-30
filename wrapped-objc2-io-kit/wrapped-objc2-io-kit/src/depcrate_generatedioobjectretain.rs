// Generated macro for IOObjectRetain (function)
macro_rules! Depcrate_generatedIOObjectRetain {
() => {
// Module: crate::generated
// Provides: {"IOObjectRetain"}
// Dependencies: {}
# [doc = " Retains an object handle previously returned by IOKitLib."] # [doc = ""] # [doc = " Gives the caller an additional reference to an existing object handle previously returned by IOKitLib."] # [doc = ""] # [doc = " Parameter `object`: The IOKit object to retain."] # [doc = ""] # [doc = " Returns: A kern_return_t error code."] # [cfg (feature = "libc")] # [inline] pub extern "C-unwind" fn IOObjectRetain (object : io_object_t) -> libc :: kern_return_t { extern "C-unwind" { fn IOObjectRetain (object : io_object_t) -> libc :: kern_return_t ; } unsafe { IOObjectRetain (object) } }
};
}
