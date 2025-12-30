// Generated macro for AudioFileComponentGetPropertyProc (type)
macro_rules! Depcrate_generatedAudioFileComponentGetPropertyProc {
() => {
// Module: crate::generated
// Provides: {"AudioFileComponentGetPropertyProc"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiofilecomponentgetpropertyproc?language=objc)"] pub type AudioFileComponentGetPropertyProc = Option < unsafe extern "C-unwind" fn (NonNull < c_void > , AudioFileComponentPropertyID , NonNull < u32 > , NonNull < c_void > ,) -> OSStatus , > ;
};
}
