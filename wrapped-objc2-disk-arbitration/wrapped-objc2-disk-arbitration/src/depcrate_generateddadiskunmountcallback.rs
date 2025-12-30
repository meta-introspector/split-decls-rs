// Generated macro for DADiskUnmountCallback (type)
macro_rules! Depcrate_generatedDADiskUnmountCallback {
() => {
// Module: crate::generated
// Provides: {"DADiskUnmountCallback"}
// Dependencies: {}
# [doc = " Type of the callback function used by DADiskUnmount()."] # [doc = ""] # [doc = " Parameter `disk`: The disk object."] # [doc = ""] # [doc = " Parameter `dissenter`: A dissenter object on failure or NULL on success."] # [doc = ""] # [doc = " Parameter `context`: The user-defined context parameter given to the unmount function."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/diskarbitration/dadiskunmountcallback?language=objc)"] # [cfg (all (feature = "DADisk" , feature = "DADissenter"))] pub type DADiskUnmountCallback = Option < unsafe extern "C-unwind" fn (NonNull < DADisk > , * const DADissenter , * mut c_void) > ;
};
}
