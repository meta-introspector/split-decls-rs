// Generated macro for ReadPacketsFDF (type)
macro_rules! Depcrate_generatedReadPacketsFDF {
() => {
// Module: crate::generated
// Provides: {"ReadPacketsFDF"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/readpacketsfdf?language=objc)"] # [cfg (feature = "objc2-core-audio-types")] pub type ReadPacketsFDF = Option < unsafe extern "C-unwind" fn (NonNull < c_void > , Boolean , NonNull < u32 > , * mut AudioStreamPacketDescription , i64 , NonNull < u32 > , NonNull < c_void > ,) -> OSStatus , > ;
};
}
