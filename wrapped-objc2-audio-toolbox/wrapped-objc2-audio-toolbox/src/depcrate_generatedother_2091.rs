// Generated macro for other_2091 (other)
macro_rules! Depcrate_generatedother_2091 {
() => {
// Module: crate::generated
// Provides: {"other_2091"}
// Dependencies: {}
extern "C-unwind" { # [doc = " implements AudioFileGetProperty."] # [doc = ""] # [doc = " Parameter `inComponent`: an AudioFileComponent"] # [doc = ""] # [doc = " Parameter `inPropertyID`: an AudioFileProperty constant."] # [doc = ""] # [doc = " Parameter `ioPropertyDataSize`: on input the size of the outPropertyData buffer. On output the number of bytes written to the buffer."] # [doc = ""] # [doc = " Parameter `outPropertyData`: the buffer in which to write the property data."] # [doc = ""] # [doc = " Returns: returns noErr if successful."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `in_component` must be a valid pointer."] # [doc = " - `io_property_data_size` must be a valid pointer."] # [doc = " - `out_property_data` must be a valid pointer."] # [cfg (feature = "AudioComponent")] pub fn AudioFileComponentGetProperty (in_component : AudioFileComponent , in_property_id : AudioFileComponentPropertyID , io_property_data_size : NonNull < u32 > , out_property_data : NonNull < c_void > ,) -> OSStatus ; }
};
}
