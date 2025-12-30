// Generated macro for AudioFileComponentOpenWithCallbacksProc (type)
macro_rules! Depcrate_generatedAudioFileComponentOpenWithCallbacksProc {
() => {
// Module: crate::generated
// Provides: {"AudioFileComponentOpenWithCallbacksProc"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiofilecomponentopenwithcallbacksproc?language=objc)"] # [cfg (feature = "AudioFile")] pub type AudioFileComponentOpenWithCallbacksProc = Option < unsafe extern "C-unwind" fn (NonNull < c_void > , NonNull < c_void > , AudioFile_ReadProc , AudioFile_WriteProc , AudioFile_GetSizeProc , AudioFile_SetSizeProc ,) -> OSStatus , > ;
};
}
