// Generated macro for AudioFileComponentGetUserDataSizeProc (type)
macro_rules! Depcrate_generatedAudioFileComponentGetUserDataSizeProc {
() => {
// Module: crate::generated
// Provides: {"AudioFileComponentGetUserDataSizeProc"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiofilecomponentgetuserdatasizeproc?language=objc)"] pub type AudioFileComponentGetUserDataSizeProc = Option < unsafe extern "C-unwind" fn (NonNull < c_void > , u32 , u32 , NonNull < u32 >) -> OSStatus > ;
};
}
