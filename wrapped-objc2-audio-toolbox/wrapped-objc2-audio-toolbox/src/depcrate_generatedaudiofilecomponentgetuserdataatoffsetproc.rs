// Generated macro for AudioFileComponentGetUserDataAtOffsetProc (type)
macro_rules! Depcrate_generatedAudioFileComponentGetUserDataAtOffsetProc {
() => {
// Module: crate::generated
// Provides: {"AudioFileComponentGetUserDataAtOffsetProc"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiofilecomponentgetuserdataatoffsetproc?language=objc)"] pub type AudioFileComponentGetUserDataAtOffsetProc = Option < unsafe extern "C-unwind" fn (NonNull < c_void > , u32 , u32 , i64 , NonNull < u32 > , NonNull < c_void > ,) -> OSStatus , > ;
};
}
