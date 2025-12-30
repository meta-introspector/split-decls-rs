// Generated macro for AudioFileComponentOpenURLProc (type)
macro_rules! Depcrate_generatedAudioFileComponentOpenURLProc {
() => {
// Module: crate::generated
// Provides: {"AudioFileComponentOpenURLProc"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiofilecomponentopenurlproc?language=objc)"] # [cfg (feature = "objc2-core-foundation")] pub type AudioFileComponentOpenURLProc = Option < unsafe extern "C-unwind" fn (NonNull < c_void > , NonNull < CFURL > , i8 , c_int) -> OSStatus > ;
};
}
