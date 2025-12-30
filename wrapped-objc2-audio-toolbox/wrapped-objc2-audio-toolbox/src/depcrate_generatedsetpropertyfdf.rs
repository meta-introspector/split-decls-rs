// Generated macro for SetPropertyFDF (type)
macro_rules! Depcrate_generatedSetPropertyFDF {
() => {
// Module: crate::generated
// Provides: {"SetPropertyFDF"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/setpropertyfdf?language=objc)"] # [cfg (feature = "AudioFile")] pub type SetPropertyFDF = Option < unsafe extern "C-unwind" fn (NonNull < c_void > , AudioFilePropertyID , u32 , NonNull < c_void > ,) -> OSStatus , > ;
};
}
