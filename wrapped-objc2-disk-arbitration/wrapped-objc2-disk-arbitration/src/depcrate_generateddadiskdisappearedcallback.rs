// Generated macro for DADiskDisappearedCallback (type)
macro_rules! Depcrate_generatedDADiskDisappearedCallback {
() => {
// Module: crate::generated
// Provides: {"DADiskDisappearedCallback"}
// Dependencies: {}
# [doc = " Type of the callback function used by DARegisterDiskDisappearedCallback()."] # [doc = ""] # [doc = " Parameter `disk`: A disk object."] # [doc = ""] # [doc = " Parameter `context`: The user-defined context parameter given to the registration function."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/diskarbitration/dadiskdisappearedcallback?language=objc)"] # [cfg (feature = "DADisk")] pub type DADiskDisappearedCallback = Option < unsafe extern "C-unwind" fn (NonNull < DADisk > , * mut c_void) > ;
};
}
