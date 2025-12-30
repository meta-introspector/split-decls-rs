// Generated macro for DADiskDescriptionChangedCallback (type)
macro_rules! Depcrate_generatedDADiskDescriptionChangedCallback {
() => {
// Module: crate::generated
// Provides: {"DADiskDescriptionChangedCallback"}
// Dependencies: {}
# [doc = " Type of the callback function used by DARegisterDiskDescriptionChangedCallback()."] # [doc = ""] # [doc = " Parameter `disk`: A disk object."] # [doc = ""] # [doc = " Parameter `keys`: A list of changed keys."] # [doc = ""] # [doc = " Parameter `context`: The user-defined context parameter given to the registration function."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/diskarbitration/dadiskdescriptionchangedcallback?language=objc)"] # [cfg (feature = "DADisk")] pub type DADiskDescriptionChangedCallback = Option < unsafe extern "C-unwind" fn (NonNull < DADisk > , NonNull < CFArray > , * mut c_void) > ;
};
}
