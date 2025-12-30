// Generated macro for ReadPacketDataFDF (type)
macro_rules! Depcrate_generatedReadPacketDataFDF {
() => {
// Module: crate::generated
// Provides: {"ReadPacketDataFDF"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/readpacketdatafdf?language=objc)"] # [cfg (feature = "objc2-core-audio-types")] pub type ReadPacketDataFDF = Option < unsafe extern "C-unwind" fn (NonNull < c_void > , Boolean , NonNull < u32 > , * mut AudioStreamPacketDescription , i64 , NonNull < u32 > , NonNull < c_void > ,) -> OSStatus , > ;
};
}
