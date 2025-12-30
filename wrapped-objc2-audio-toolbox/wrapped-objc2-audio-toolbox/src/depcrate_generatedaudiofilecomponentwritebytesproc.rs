// Generated macro for AudioFileComponentWriteBytesProc (type)
macro_rules! Depcrate_generatedAudioFileComponentWriteBytesProc {
() => {
// Module: crate::generated
// Provides: {"AudioFileComponentWriteBytesProc"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiofilecomponentwritebytesproc?language=objc)"] pub type AudioFileComponentWriteBytesProc = Option < unsafe extern "C-unwind" fn (NonNull < c_void > , Boolean , i64 , NonNull < u32 > , NonNull < c_void > ,) -> OSStatus , > ;
};
}
