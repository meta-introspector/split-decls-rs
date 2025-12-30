// Generated macro for DADiskClaimCallback (type)
macro_rules! Depcrate_generatedDADiskClaimCallback {
() => {
// Module: crate::generated
// Provides: {"DADiskClaimCallback"}
// Dependencies: {}
# [doc = " Type of the callback function used by DADiskClaim()."] # [doc = ""] # [doc = " Parameter `disk`: The disk object."] # [doc = ""] # [doc = " Parameter `dissenter`: A dissenter object on failure or NULL on success."] # [doc = ""] # [doc = " Parameter `context`: The user-defined context parameter given to the claim function."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/diskarbitration/dadiskclaimcallback?language=objc)"] # [cfg (all (feature = "DADisk" , feature = "DADissenter"))] pub type DADiskClaimCallback = Option < unsafe extern "C-unwind" fn (NonNull < DADisk > , * const DADissenter , * mut c_void) > ;
};
}
