// Generated macro for AudioFileComponentReadPacketDataProc (type)
macro_rules! Depcrate_generatedAudioFileComponentReadPacketDataProc {
() => {
// Module: crate::generated
// Provides: {"AudioFileComponentReadPacketDataProc"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiofilecomponentreadpacketdataproc?language=objc)"] # [cfg (feature = "objc2-core-audio-types")] pub type AudioFileComponentReadPacketDataProc = Option < unsafe extern "C-unwind" fn (NonNull < c_void > , Boolean , NonNull < u32 > , * mut AudioStreamPacketDescription , i64 , NonNull < u32 > , NonNull < c_void > ,) -> OSStatus , > ;
};
}
