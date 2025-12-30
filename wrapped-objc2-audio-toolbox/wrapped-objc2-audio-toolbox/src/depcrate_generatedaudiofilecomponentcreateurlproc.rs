// Generated macro for AudioFileComponentCreateURLProc (type)
macro_rules! Depcrate_generatedAudioFileComponentCreateURLProc {
() => {
// Module: crate::generated
// Provides: {"AudioFileComponentCreateURLProc"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiofilecomponentcreateurlproc?language=objc)"] # [cfg (all (feature = "objc2-core-audio-types" , feature = "objc2-core-foundation"))] pub type AudioFileComponentCreateURLProc = Option < unsafe extern "C-unwind" fn (NonNull < c_void > , NonNull < CFURL > , NonNull < AudioStreamBasicDescription > , u32 ,) -> OSStatus , > ;
};
}
