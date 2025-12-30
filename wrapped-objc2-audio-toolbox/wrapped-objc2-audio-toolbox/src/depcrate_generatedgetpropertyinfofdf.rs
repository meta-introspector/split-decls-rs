// Generated macro for GetPropertyInfoFDF (type)
macro_rules! Depcrate_generatedGetPropertyInfoFDF {
() => {
// Module: crate::generated
// Provides: {"GetPropertyInfoFDF"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/getpropertyinfofdf?language=objc)"] # [cfg (feature = "AudioFile")] pub type GetPropertyInfoFDF = Option < unsafe extern "C-unwind" fn (NonNull < c_void > , AudioFilePropertyID , * mut u32 , * mut u32 ,) -> OSStatus , > ;
};
}
