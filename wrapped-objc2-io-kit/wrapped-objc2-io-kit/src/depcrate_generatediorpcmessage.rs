// Generated macro for IORPCMessage (struct)
macro_rules! Depcrate_generatedIORPCMessage {
() => {
// Module: crate::generated
// Provides: {"IORPCMessage"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/iokit/iorpcmessage?language=objc)"] # [repr (C , packed (4))] # [derive (Clone , Copy , Debug , PartialEq , Default)] pub struct IORPCMessage { pub msgid : u64 , pub flags : u64 , pub objectRefs : u64 , pub objects : [OSObjectRef ; 0] , }
};
}
