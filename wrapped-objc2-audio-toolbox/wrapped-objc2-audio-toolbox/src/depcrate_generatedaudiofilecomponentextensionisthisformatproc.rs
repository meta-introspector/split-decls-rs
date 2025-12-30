// Generated macro for AudioFileComponentExtensionIsThisFormatProc (type)
macro_rules! Depcrate_generatedAudioFileComponentExtensionIsThisFormatProc {
() => {
// Module: crate::generated
// Provides: {"AudioFileComponentExtensionIsThisFormatProc"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiofilecomponentextensionisthisformatproc?language=objc)"] # [cfg (feature = "objc2-core-foundation")] pub type AudioFileComponentExtensionIsThisFormatProc = Option < unsafe extern "C-unwind" fn (NonNull < c_void > , NonNull < CFString > , NonNull < u32 >) -> OSStatus , > ;
};
}
