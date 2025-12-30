// Generated macro for AudioFileComponentGetGlobalInfoSizeProc (type)
macro_rules! Depcrate_generatedAudioFileComponentGetGlobalInfoSizeProc {
() => {
// Module: crate::generated
// Provides: {"AudioFileComponentGetGlobalInfoSizeProc"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiofilecomponentgetglobalinfosizeproc?language=objc)"] pub type AudioFileComponentGetGlobalInfoSizeProc = Option < unsafe extern "C-unwind" fn (NonNull < c_void > , AudioFileComponentPropertyID , u32 , * const c_void , NonNull < u32 > ,) -> OSStatus , > ;
};
}
