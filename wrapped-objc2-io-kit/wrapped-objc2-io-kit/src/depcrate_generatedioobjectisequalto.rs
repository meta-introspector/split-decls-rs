// Generated macro for IOObjectIsEqualTo (function)
macro_rules! Depcrate_generatedIOObjectIsEqualTo {
() => {
// Module: crate::generated
// Provides: {"IOObjectIsEqualTo"}
// Dependencies: {}
# [doc = " Checks two object handles to see if they represent the same kernel object."] # [doc = ""] # [doc = " If two object handles are returned by IOKitLib functions, this function will compare them to see if they represent the same kernel object."] # [doc = ""] # [doc = " Parameter `object`: An IOKit object."] # [doc = ""] # [doc = " Parameter `anObject`: Another IOKit object."] # [doc = ""] # [doc = " Returns: If both object handles are valid, and represent the same object in the kernel true is returned, otherwise false."] # [cfg (feature = "libc")] # [inline] pub extern "C-unwind" fn IOObjectIsEqualTo (object : io_object_t , an_object : io_object_t) -> bool { extern "C-unwind" { fn IOObjectIsEqualTo (object : io_object_t , an_object : io_object_t) -> libc :: boolean_t ; } let ret = unsafe { IOObjectIsEqualTo (object , an_object) } ; ret != 0 }
};
}
