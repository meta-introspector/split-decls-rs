// Generated macro for AudioFileComponentGetPropertyInfoProc (type)
macro_rules! Depcrate_generatedAudioFileComponentGetPropertyInfoProc {
() => {
// Module: crate::generated
// Provides: {"AudioFileComponentGetPropertyInfoProc"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiofilecomponentgetpropertyinfoproc?language=objc)"] pub type AudioFileComponentGetPropertyInfoProc = Option < unsafe extern "C-unwind" fn (NonNull < c_void > , AudioFileComponentPropertyID , * mut u32 , * mut u32 ,) -> OSStatus , > ;
};
}
