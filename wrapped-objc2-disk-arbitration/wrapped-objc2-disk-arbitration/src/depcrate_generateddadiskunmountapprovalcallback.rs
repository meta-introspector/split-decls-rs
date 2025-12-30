// Generated macro for DADiskUnmountApprovalCallback (type)
macro_rules! Depcrate_generatedDADiskUnmountApprovalCallback {
() => {
// Module: crate::generated
// Provides: {"DADiskUnmountApprovalCallback"}
// Dependencies: {}
# [doc = " Type of the callback function used by DARegisterDiskUnmountApprovalCallback()."] # [doc = ""] # [doc = " Parameter `disk`: A disk object."] # [doc = ""] # [doc = " Parameter `context`: The user-defined context parameter given to the registration function."] # [doc = ""] # [doc = " Returns: A dissenter reference.  Pass NULL to approve."] # [doc = ""] # [doc = " The caller of this callback receives a reference to the returned object.  The"] # [doc = " caller also implicitly retains the object and is responsible for releasing it"] # [doc = " with CFRelease()."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/diskarbitration/dadiskunmountapprovalcallback?language=objc)"] # [cfg (all (feature = "DADisk" , feature = "DADissenter"))] pub type DADiskUnmountApprovalCallback = Option < unsafe extern "C-unwind" fn (NonNull < DADisk > , * mut c_void) -> * const DADissenter > ;
};
}
