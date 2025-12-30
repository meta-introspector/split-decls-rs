// Generated macro for IOServiceInterestContent (struct)
macro_rules! Depcrate_generatedIOServiceInterestContent {
() => {
// Module: crate::generated
// Provides: {"IOServiceInterestContent"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/iokit/ioserviceinterestcontent?language=objc)"] # [cfg (feature = "libc")] # [repr (C , packed (4))] # [derive (Clone , Copy , Debug , PartialEq)] pub struct IOServiceInterestContent { pub messageType : libc :: natural_t , pub messageArgument : [* mut c_void ; 1] , }
};
}
