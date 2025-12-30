// Generated macro for other_4794 (other)
macro_rules! Depcrate_generatedother_4794 {
() => {
// Module: crate::generated
// Provides: {"other_4794"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Return the class name of an IOKit object."] # [doc = ""] # [doc = " This function uses the OSMetaClass system in the kernel to derive the name of the class the object is an instance of."] # [doc = ""] # [doc = " Parameter `object`: The IOKit object."] # [doc = ""] # [doc = " Parameter `className`: Caller allocated buffer to receive the name string."] # [doc = ""] # [doc = " Returns: A kern_return_t error code."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `class_name` must be a valid pointer."] # [cfg (feature = "libc")] pub fn IOObjectGetClass (object : io_object_t , class_name : * mut io_name_t) -> libc :: kern_return_t ; }
};
}
