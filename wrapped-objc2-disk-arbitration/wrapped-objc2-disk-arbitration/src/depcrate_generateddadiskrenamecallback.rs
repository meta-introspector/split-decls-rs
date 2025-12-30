// Generated macro for DADiskRenameCallback (type)
macro_rules! Depcrate_generatedDADiskRenameCallback {
() => {
// Module: crate::generated
// Provides: {"DADiskRenameCallback"}
// Dependencies: {}
# [doc = " Type of the callback function used by DADiskRename()."] # [doc = ""] # [doc = " Parameter `disk`: The disk object."] # [doc = ""] # [doc = " Parameter `dissenter`: A dissenter object on failure or NULL on success."] # [doc = ""] # [doc = " Parameter `context`: The user-defined context parameter given to the rename function."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/diskarbitration/dadiskrenamecallback?language=objc)"] # [cfg (all (feature = "DADisk" , feature = "DADissenter"))] pub type DADiskRenameCallback = Option < unsafe extern "C-unwind" fn (NonNull < DADisk > , * const DADissenter , * mut c_void) > ;
};
}
