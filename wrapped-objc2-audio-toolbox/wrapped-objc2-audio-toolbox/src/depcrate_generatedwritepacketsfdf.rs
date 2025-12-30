// Generated macro for WritePacketsFDF (type)
macro_rules! Depcrate_generatedWritePacketsFDF {
() => {
// Module: crate::generated
// Provides: {"WritePacketsFDF"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/writepacketsfdf?language=objc)"] # [cfg (feature = "objc2-core-audio-types")] pub type WritePacketsFDF = Option < unsafe extern "C-unwind" fn (NonNull < c_void > , Boolean , u32 , * const AudioStreamPacketDescription , i64 , NonNull < u32 > , NonNull < c_void > ,) -> OSStatus , > ;
};
}
