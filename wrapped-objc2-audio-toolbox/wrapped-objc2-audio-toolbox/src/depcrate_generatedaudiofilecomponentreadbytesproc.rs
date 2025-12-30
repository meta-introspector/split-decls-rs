// Generated macro for AudioFileComponentReadBytesProc (type)
macro_rules! Depcrate_generatedAudioFileComponentReadBytesProc {
() => {
// Module: crate::generated
// Provides: {"AudioFileComponentReadBytesProc"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiofilecomponentreadbytesproc?language=objc)"] pub type AudioFileComponentReadBytesProc = Option < unsafe extern "C-unwind" fn (NonNull < c_void > , Boolean , i64 , NonNull < u32 > , NonNull < c_void > ,) -> OSStatus , > ;
};
}
