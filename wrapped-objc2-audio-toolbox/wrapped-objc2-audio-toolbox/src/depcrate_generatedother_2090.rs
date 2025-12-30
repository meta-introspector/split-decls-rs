// Generated macro for other_2090 (other)
macro_rules! Depcrate_generatedother_2090 {
() => {
// Module: crate::generated
// Provides: {"other_2090"}
// Dependencies: {}
extern "C-unwind" { # [doc = " implements AudioFileGetPropertyInfo."] # [doc = ""] # [doc = " Parameter `inComponent`: an AudioFileComponent"] # [doc = ""] # [doc = " Parameter `inPropertyID`: an AudioFileProperty constant."] # [doc = ""] # [doc = " Parameter `outPropertySize`: the size in bytes of the current value of the property. In order to get the property value,"] # [doc = " you will need a buffer of this size."] # [doc = ""] # [doc = " Parameter `outWritable`: will be set to 1 if writable, or 0 if read only."] # [doc = ""] # [doc = " Returns: returns noErr if successful."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `in_component` must be a valid pointer."] # [doc = " - `out_property_size` must be a valid pointer or null."] # [doc = " - `out_writable` must be a valid pointer or null."] # [cfg (feature = "AudioComponent")] pub fn AudioFileComponentGetPropertyInfo (in_component : AudioFileComponent , in_property_id : AudioFileComponentPropertyID , out_property_size : * mut u32 , out_writable : * mut u32 ,) -> OSStatus ; }
};
}
