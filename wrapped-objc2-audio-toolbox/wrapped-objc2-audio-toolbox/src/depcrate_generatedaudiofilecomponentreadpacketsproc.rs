// Generated macro for AudioFileComponentReadPacketsProc (type)
macro_rules! Depcrate_generatedAudioFileComponentReadPacketsProc {
() => {
// Module: crate::generated
// Provides: {"AudioFileComponentReadPacketsProc"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiofilecomponentreadpacketsproc?language=objc)"] # [cfg (feature = "objc2-core-audio-types")] pub type AudioFileComponentReadPacketsProc = Option < unsafe extern "C-unwind" fn (NonNull < c_void > , Boolean , NonNull < u32 > , * mut AudioStreamPacketDescription , i64 , NonNull < u32 > , NonNull < c_void > ,) -> OSStatus , > ;
};
}
