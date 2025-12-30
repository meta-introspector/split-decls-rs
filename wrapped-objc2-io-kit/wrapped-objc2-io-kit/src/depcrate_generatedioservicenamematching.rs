// Generated macro for IOServiceNameMatching (function)
macro_rules! Depcrate_generatedIOServiceNameMatching {
() => {
// Module: crate::generated
// Provides: {"IOServiceNameMatching"}
// Dependencies: {}
# [doc = " Create a matching dictionary that specifies an IOService name match."] # [doc = ""] # [doc = " A common matching criteria for IOService is based on its name. IOServiceNameMatching will create a matching dictionary that specifies an IOService with a given name. Some IOServices created from the device tree will perform name matching on the standard compatible, name, model properties."] # [doc = ""] # [doc = " Parameter `name`: The IOService name, as a const C-string."] # [doc = ""] # [doc = " Returns: The matching dictionary created, is returned on success, or zero on failure. The dictionary is commonly passed to IOServiceGetMatchingServices or IOServiceAddNotification which will consume a reference, otherwise it should be released with CFRelease by the caller."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `name` must be a valid pointer."] # [doc = " - The returned generic must be of the correct type."] # [doc = " - The returned generic must be of the correct type."] # [inline] pub unsafe extern "C-unwind" fn IOServiceNameMatching (name : * const c_char ,) -> Option < CFRetained < CFMutableDictionary > > { extern "C-unwind" { fn IOServiceNameMatching (name : * const c_char) -> Option < NonNull < CFMutableDictionary > > ; } let ret = unsafe { IOServiceNameMatching (name) } ; ret . map (| ret | unsafe { CFRetained :: from_raw (ret) }) }
};
}
