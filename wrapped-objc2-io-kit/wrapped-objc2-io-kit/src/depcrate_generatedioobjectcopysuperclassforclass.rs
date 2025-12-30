// Generated macro for IOObjectCopySuperclassForClass (function)
macro_rules! Depcrate_generatedIOObjectCopySuperclassForClass {
() => {
// Module: crate::generated
// Provides: {"IOObjectCopySuperclassForClass"}
// Dependencies: {}
# [doc = " Return the superclass name of the given class."] # [doc = ""] # [doc = " This function uses the OSMetaClass system in the kernel to derive the name of the superclass of the class."] # [doc = ""] # [doc = " Parameter `classname`: The name of the class as a CFString."] # [doc = ""] # [doc = " Returns: The resulting CFStringRef. This should be released by the caller. If there is no superclass, or a valid class name is not passed in, then NULL is returned."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `classname` might not allow `None`."] # [inline] pub unsafe extern "C-unwind" fn IOObjectCopySuperclassForClass (classname : Option < & CFString > ,) -> Option < CFRetained < CFString > > { extern "C-unwind" { fn IOObjectCopySuperclassForClass (classname : Option < & CFString > ,) -> Option < NonNull < CFString > > ; } let ret = unsafe { IOObjectCopySuperclassForClass (classname) } ; ret . map (| ret | unsafe { CFRetained :: from_raw (ret) }) }
};
}
