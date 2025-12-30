// Generated macro for other_2105 (other)
macro_rules! Depcrate_generatedother_2105 {
() => {
// Module: crate::generated
// Provides: {"other_2105"}
// Dependencies: {}
extern "C-unwind" { # [doc = " implements AudioFileGetGlobalInfo."] # [doc = ""] # [doc = " Parameter `inComponent`: an AudioFileComponent"] # [doc = ""] # [doc = " Parameter `inPropertyID`: an AudioFileGlobalInfo property constant."] # [doc = ""] # [doc = " Parameter `inSpecifierSize`: The size of the specifier data."] # [doc = ""] # [doc = " Parameter `inSpecifier`: A specifier is a buffer of data used as an input argument to some of the global info properties."] # [doc = ""] # [doc = " Parameter `ioPropertyDataSize`: on input the size of the outPropertyData buffer. On output the number of bytes written to the buffer."] # [doc = ""] # [doc = " Parameter `outPropertyData`: the buffer in which to write the property data."] # [doc = ""] # [doc = " Returns: returns noErr if successful."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `in_component` must be a valid pointer."] # [doc = " - `in_specifier` must be a valid pointer or null."] # [doc = " - `io_property_data_size` must be a valid pointer."] # [doc = " - `out_property_data` must be a valid pointer."] # [cfg (feature = "AudioComponent")] pub fn AudioFileComponentGetGlobalInfo (in_component : AudioFileComponent , in_property_id : AudioFileComponentPropertyID , in_specifier_size : u32 , in_specifier : * const c_void , io_property_data_size : NonNull < u32 > , out_property_data : NonNull < c_void > ,) -> OSStatus ; }
};
}
