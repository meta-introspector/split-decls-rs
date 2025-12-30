// Generated macro for GetPropertyFDF (type)
macro_rules! Depcrate_generatedGetPropertyFDF {
() => {
// Module: crate::generated
// Provides: {"GetPropertyFDF"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/getpropertyfdf?language=objc)"] # [cfg (feature = "AudioFile")] pub type GetPropertyFDF = Option < unsafe extern "C-unwind" fn (NonNull < c_void > , AudioFilePropertyID , NonNull < u32 > , NonNull < c_void > ,) -> OSStatus , > ;
};
}
