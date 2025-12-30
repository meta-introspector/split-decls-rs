// Generated macro for DADiskAppearedCallback (type)
macro_rules! Depcrate_generatedDADiskAppearedCallback {
() => {
// Module: crate::generated
// Provides: {"DADiskAppearedCallback"}
// Dependencies: {}
# [doc = " Type of the callback function used by DARegisterDiskAppearedCallback()."] # [doc = ""] # [doc = " Parameter `disk`: A disk object."] # [doc = ""] # [doc = " Parameter `context`: The user-defined context parameter given to the registration function."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/diskarbitration/dadiskappearedcallback?language=objc)"] # [cfg (feature = "DADisk")] pub type DADiskAppearedCallback = Option < unsafe extern "C-unwind" fn (NonNull < DADisk > , * mut c_void) > ;
};
}
