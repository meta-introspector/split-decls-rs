// Generated macro for DADiskPeekCallback (type)
macro_rules! Depcrate_generatedDADiskPeekCallback {
() => {
// Module: crate::generated
// Provides: {"DADiskPeekCallback"}
// Dependencies: {}
# [doc = " Type of the callback function used by DARegisterDiskPeekCallback()."] # [doc = ""] # [doc = " Parameter `disk`: A disk object."] # [doc = ""] # [doc = " Parameter `context`: The user-defined context parameter given to the registration function."] # [doc = ""] # [doc = " The peek callback functions are called in a specific order, from lowest order to highest"] # [doc = " order.  DADiskClaim() could be used here to claim the disk object and DADiskSetOptions()"] # [doc = " could be used here to set up options on the disk object."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/diskarbitration/dadiskpeekcallback?language=objc)"] # [cfg (feature = "DADisk")] pub type DADiskPeekCallback = Option < unsafe extern "C-unwind" fn (NonNull < DADisk > , * mut c_void) > ;
};
}
