// Generated macro for AudioFileComponentGetGlobalInfoProc (type)
macro_rules! Depcrate_generatedAudioFileComponentGetGlobalInfoProc {
() => {
// Module: crate::generated
// Provides: {"AudioFileComponentGetGlobalInfoProc"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiofilecomponentgetglobalinfoproc?language=objc)"] pub type AudioFileComponentGetGlobalInfoProc = Option < unsafe extern "C-unwind" fn (NonNull < c_void > , AudioFileComponentPropertyID , u32 , * const c_void , NonNull < u32 > , NonNull < c_void > ,) -> OSStatus , > ;
};
}
