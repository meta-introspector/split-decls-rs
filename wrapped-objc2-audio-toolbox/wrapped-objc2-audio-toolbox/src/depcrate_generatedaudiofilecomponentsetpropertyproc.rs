// Generated macro for AudioFileComponentSetPropertyProc (type)
macro_rules! Depcrate_generatedAudioFileComponentSetPropertyProc {
() => {
// Module: crate::generated
// Provides: {"AudioFileComponentSetPropertyProc"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiofilecomponentsetpropertyproc?language=objc)"] pub type AudioFileComponentSetPropertyProc = Option < unsafe extern "C-unwind" fn (NonNull < c_void > , AudioFileComponentPropertyID , u32 , NonNull < c_void > ,) -> OSStatus , > ;
};
}
