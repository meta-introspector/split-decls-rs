// Generated macro for AudioFileComponentInitializeWithCallbacksProc (type)
macro_rules! Depcrate_generatedAudioFileComponentInitializeWithCallbacksProc {
() => {
// Module: crate::generated
// Provides: {"AudioFileComponentInitializeWithCallbacksProc"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiofilecomponentinitializewithcallbacksproc?language=objc)"] # [cfg (all (feature = "AudioFile" , feature = "objc2-core-audio-types"))] pub type AudioFileComponentInitializeWithCallbacksProc = Option < unsafe extern "C-unwind" fn (NonNull < c_void > , NonNull < c_void > , AudioFile_ReadProc , AudioFile_WriteProc , AudioFile_GetSizeProc , AudioFile_SetSizeProc , u32 , NonNull < AudioStreamBasicDescription > , u32 ,) -> OSStatus , > ;
};
}
