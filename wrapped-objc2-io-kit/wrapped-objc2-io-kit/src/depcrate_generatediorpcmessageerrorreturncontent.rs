// Generated macro for IORPCMessageErrorReturnContent (struct)
macro_rules! Depcrate_generatedIORPCMessageErrorReturnContent {
() => {
// Module: crate::generated
// Provides: {"IORPCMessageErrorReturnContent"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/iokit/iorpcmessageerrorreturncontent?language=objc)"] # [cfg (feature = "libc")] # [repr (C)] # [derive (Clone , Copy , Debug , PartialEq , Default)] pub struct IORPCMessageErrorReturnContent { pub hdr : IORPCMessage , pub result : libc :: kern_return_t , pub pad : u32 , }
};
}
