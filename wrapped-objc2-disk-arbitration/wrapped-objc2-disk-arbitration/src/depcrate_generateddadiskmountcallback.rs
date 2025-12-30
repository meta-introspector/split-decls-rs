// Generated macro for DADiskMountCallback (type)
macro_rules! Depcrate_generatedDADiskMountCallback {
() => {
// Module: crate::generated
// Provides: {"DADiskMountCallback"}
// Dependencies: {}
# [doc = " Type of the callback function used by DADiskMount()."] # [doc = ""] # [doc = " Parameter `disk`: The disk object."] # [doc = ""] # [doc = " Parameter `dissenter`: A dissenter object on failure or NULL on success."] # [doc = ""] # [doc = " Parameter `context`: The user-defined context parameter given to the mount function."] # [doc = ""] # [doc = " If the disk is already mounted, then status code in the dissenter object will be set to kDAReturnBusy"] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/diskarbitration/dadiskmountcallback?language=objc)"] # [cfg (all (feature = "DADisk" , feature = "DADissenter"))] pub type DADiskMountCallback = Option < unsafe extern "C-unwind" fn (NonNull < DADisk > , * const DADissenter , * mut c_void) > ;
};
}
