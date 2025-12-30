// Generated macro for IOObjectCopyBundleIdentifierForClass (function)
macro_rules! Depcrate_generatedIOObjectCopyBundleIdentifierForClass {
() => {
// Module: crate::generated
// Provides: {"IOObjectCopyBundleIdentifierForClass"}
// Dependencies: {}
# [doc = " Return the bundle identifier of the given class."] # [doc = ""] # [doc = " This function uses the OSMetaClass system in the kernel to derive the name of the kmod, which is the same as the bundle identifier."] # [doc = ""] # [doc = " Parameter `classname`: The name of the class as a CFString."] # [doc = ""] # [doc = " Returns: The resulting CFStringRef. This should be released by the caller. If a valid class name is not passed in, then NULL is returned."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `classname` might not allow `None`."] # [inline] pub unsafe extern "C-unwind" fn IOObjectCopyBundleIdentifierForClass (classname : Option < & CFString > ,) -> Option < CFRetained < CFString > > { extern "C-unwind" { fn IOObjectCopyBundleIdentifierForClass (classname : Option < & CFString > ,) -> Option < NonNull < CFString > > ; } let ret = unsafe { IOObjectCopyBundleIdentifierForClass (classname) } ; ret . map (| ret | unsafe { CFRetained :: from_raw (ret) }) }
};
}
