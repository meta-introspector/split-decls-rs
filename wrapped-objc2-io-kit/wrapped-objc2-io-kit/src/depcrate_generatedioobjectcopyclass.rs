// Generated macro for IOObjectCopyClass (function)
macro_rules! Depcrate_generatedIOObjectCopyClass {
() => {
// Module: crate::generated
// Provides: {"IOObjectCopyClass"}
// Dependencies: {}
# [doc = " Return the class name of an IOKit object."] # [doc = ""] # [doc = " This function does the same thing as IOObjectGetClass, but returns the result as a CFStringRef."] # [doc = ""] # [doc = " Parameter `object`: The IOKit object."] # [doc = ""] # [doc = " Returns: The resulting CFStringRef. This should be released by the caller. If a valid object is not passed in, then NULL is returned."] # [cfg (feature = "libc")] # [inline] pub extern "C-unwind" fn IOObjectCopyClass (object : io_object_t) -> Option < CFRetained < CFString > > { extern "C-unwind" { fn IOObjectCopyClass (object : io_object_t) -> Option < NonNull < CFString > > ; } let ret = unsafe { IOObjectCopyClass (object) } ; ret . map (| ret | unsafe { CFRetained :: from_raw (ret) }) }
};
}
