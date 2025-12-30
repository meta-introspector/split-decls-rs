// Generated macro for DADiskClaimReleaseCallback (type)
macro_rules! Depcrate_generatedDADiskClaimReleaseCallback {
() => {
// Module: crate::generated
// Provides: {"DADiskClaimReleaseCallback"}
// Dependencies: {}
# [doc = " Type of the callback function used by DADiskClaim()."] # [doc = ""] # [doc = " Parameter `disk`: The disk object."] # [doc = ""] # [doc = " Parameter `context`: The user-defined context parameter given to the claim function."] # [doc = ""] # [doc = " Returns: A dissenter reference.  Pass NULL to release claim."] # [doc = ""] # [doc = " The caller of this callback receives a reference to the returned object.  The"] # [doc = " caller also implicitly retains the object and is responsible for releasing it"] # [doc = " with CFRelease()."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/diskarbitration/dadiskclaimreleasecallback?language=objc)"] # [cfg (all (feature = "DADisk" , feature = "DADissenter"))] pub type DADiskClaimReleaseCallback = Option < unsafe extern "C-unwind" fn (NonNull < DADisk > , * mut c_void) -> * const DADissenter > ;
};
}
