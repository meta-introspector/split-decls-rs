// Generated macro for AudioFileComponentWritePacketsProc (type)
macro_rules! Depcrate_generatedAudioFileComponentWritePacketsProc {
() => {
// Module: crate::generated
// Provides: {"AudioFileComponentWritePacketsProc"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiofilecomponentwritepacketsproc?language=objc)"] # [cfg (feature = "objc2-core-audio-types")] pub type AudioFileComponentWritePacketsProc = Option < unsafe extern "C-unwind" fn (NonNull < c_void > , Boolean , u32 , * const AudioStreamPacketDescription , i64 , NonNull < u32 > , NonNull < c_void > ,) -> OSStatus , > ;
};
}
