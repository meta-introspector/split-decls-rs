// Generated macro for IORegistryEntryIDMatching (function)
macro_rules! Depcrate_generatedIORegistryEntryIDMatching {
() => {
// Module: crate::generated
// Provides: {"IORegistryEntryIDMatching"}
// Dependencies: {}
# [doc = " Create a matching dictionary that specifies an IOService match based on a registry entry ID."] # [doc = ""] # [doc = " This function creates a matching dictionary that will match a registered, active IOService found with the given registry entry ID. The entry ID for a registry entry is returned by IORegistryEntryGetRegistryEntryID()."] # [doc = ""] # [doc = " Parameter `entryID`: The registry entry ID to be found."] # [doc = ""] # [doc = " Returns: The matching dictionary created, is returned on success, or zero on failure. The dictionary is commonly passed to IOServiceGetMatchingServices or IOServiceAddNotification which will consume a reference, otherwise it should be released with CFRelease by the caller."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - The returned generic must be of the correct type."] # [doc = " - The returned generic must be of the correct type."] # [inline] pub unsafe extern "C-unwind" fn IORegistryEntryIDMatching (entry_id : u64 ,) -> Option < CFRetained < CFMutableDictionary > > { extern "C-unwind" { fn IORegistryEntryIDMatching (entry_id : u64) -> Option < NonNull < CFMutableDictionary > > ; } let ret = unsafe { IORegistryEntryIDMatching (entry_id) } ; ret . map (| ret | unsafe { CFRetained :: from_raw (ret) }) }
};
}
