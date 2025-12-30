// Generated macro for AudioFileComponentGetUserDataSize64Proc (type)
macro_rules! Depcrate_generatedAudioFileComponentGetUserDataSize64Proc {
() => {
// Module: crate::generated
// Provides: {"AudioFileComponentGetUserDataSize64Proc"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiofilecomponentgetuserdatasize64proc?language=objc)"] pub type AudioFileComponentGetUserDataSize64Proc = Option < unsafe extern "C-unwind" fn (NonNull < c_void > , u32 , u32 , NonNull < u64 >) -> OSStatus > ;
};
}
