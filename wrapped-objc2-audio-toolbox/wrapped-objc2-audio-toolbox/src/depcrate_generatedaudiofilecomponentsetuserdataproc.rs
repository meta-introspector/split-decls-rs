// Generated macro for AudioFileComponentSetUserDataProc (type)
macro_rules! Depcrate_generatedAudioFileComponentSetUserDataProc {
() => {
// Module: crate::generated
// Provides: {"AudioFileComponentSetUserDataProc"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiofilecomponentsetuserdataproc?language=objc)"] pub type AudioFileComponentSetUserDataProc = Option < unsafe extern "C-unwind" fn (NonNull < c_void > , u32 , u32 , u32 , NonNull < c_void >) -> OSStatus , > ;
};
}
