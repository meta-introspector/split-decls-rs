// Generated macro for AudioFileComponentGetUserDataProc (type)
macro_rules! Depcrate_generatedAudioFileComponentGetUserDataProc {
() => {
// Module: crate::generated
// Provides: {"AudioFileComponentGetUserDataProc"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiofilecomponentgetuserdataproc?language=objc)"] pub type AudioFileComponentGetUserDataProc = Option < unsafe extern "C-unwind" fn (NonNull < c_void > , u32 , u32 , NonNull < u32 > , NonNull < c_void > ,) -> OSStatus , > ;
};
}
