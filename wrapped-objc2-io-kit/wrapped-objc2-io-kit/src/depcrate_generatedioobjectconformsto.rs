// Generated macro for IOObjectConformsTo (function)
macro_rules! Depcrate_generatedIOObjectConformsTo {
() => {
// Module: crate::generated
// Provides: {"IOObjectConformsTo"}
// Dependencies: {}
# [doc = " Performs an OSDynamicCast operation on an IOKit object."] # [doc = ""] # [doc = " This function uses the OSMetaClass system in the kernel to determine if the object will dynamic cast to a class, specified as a C-string. In other words, if the object is of that class or a subclass."] # [doc = ""] # [doc = " Parameter `object`: An IOKit object."] # [doc = ""] # [doc = " Parameter `className`: The name of the class, as a C-string."] # [doc = ""] # [doc = " Returns: If the object handle is valid, and represents an object in the kernel that dynamic casts to the class true is returned, otherwise false."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `class_name` must be a valid pointer."] # [cfg (feature = "libc")] # [inline] pub unsafe extern "C-unwind" fn IOObjectConformsTo (object : io_object_t , class_name : * mut io_name_t ,) -> bool { extern "C-unwind" { fn IOObjectConformsTo (object : io_object_t , class_name : * mut io_name_t) -> libc :: boolean_t ; } let ret = unsafe { IOObjectConformsTo (object , class_name) } ; ret != 0 }
};
}
