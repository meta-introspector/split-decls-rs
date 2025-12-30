// Generated macro for IOServiceMatching (function)
macro_rules! Depcrate_generatedIOServiceMatching {
() => {
// Module: crate::generated
// Provides: {"IOServiceMatching"}
// Dependencies: {}
# [doc = " Create a matching dictionary that specifies an IOService class match."] # [doc = ""] # [doc = " A very common matching criteria for IOService is based on its class. IOServiceMatching will create a matching dictionary that specifies any IOService of a class, or its subclasses. The class is specified by C-string name."] # [doc = ""] # [doc = " Parameter `name`: The class name, as a const C-string. Class matching is successful on IOService's of this class or any subclass."] # [doc = ""] # [doc = " Returns: The matching dictionary created, is returned on success, or zero on failure. The dictionary is commonly passed to IOServiceGetMatchingServices or IOServiceAddNotification which will consume a reference, otherwise it should be released with CFRelease by the caller."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `name` must be a valid pointer."] # [doc = " - The returned generic must be of the correct type."] # [doc = " - The returned generic must be of the correct type."] # [inline] pub unsafe extern "C-unwind" fn IOServiceMatching (name : * const c_char ,) -> Option < CFRetained < CFMutableDictionary > > { extern "C-unwind" { fn IOServiceMatching (name : * const c_char) -> Option < NonNull < CFMutableDictionary > > ; } let ret = unsafe { IOServiceMatching (name) } ; ret . map (| ret | unsafe { CFRetained :: from_raw (ret) }) }
};
}
