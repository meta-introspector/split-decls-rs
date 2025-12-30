// Generated macro for DADiskEjectCallback (type)
macro_rules! Depcrate_generatedDADiskEjectCallback {
() => {
// Module: crate::generated
// Provides: {"DADiskEjectCallback"}
// Dependencies: {}
# [doc = " Type of the callback function used by DADiskEject()."] # [doc = ""] # [doc = " Parameter `disk`: The disk object."] # [doc = ""] # [doc = " Parameter `dissenter`: A dissenter object on failure or NULL on success."] # [doc = ""] # [doc = " Parameter `context`: The user-defined context parameter given to the eject function."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/diskarbitration/dadiskejectcallback?language=objc)"] # [cfg (all (feature = "DADisk" , feature = "DADissenter"))] pub type DADiskEjectCallback = Option < unsafe extern "C-unwind" fn (NonNull < DADisk > , * const DADissenter , * mut c_void) > ;
};
}
