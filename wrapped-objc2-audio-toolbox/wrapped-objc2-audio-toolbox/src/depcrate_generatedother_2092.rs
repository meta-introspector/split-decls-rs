// Generated macro for other_2092 (other)
macro_rules! Depcrate_generatedother_2092 {
() => {
// Module: crate::generated
// Provides: {"other_2092"}
// Dependencies: {}
extern "C-unwind" { # [doc = " implements AudioFileSetProperty."] # [doc = ""] # [doc = " Parameter `inComponent`: an AudioFileComponent"] # [doc = ""] # [doc = " Parameter `inPropertyID`: an AudioFileProperty constant."] # [doc = ""] # [doc = " Parameter `inPropertyDataSize`: the size of the property data."] # [doc = ""] # [doc = " Parameter `inPropertyData`: the buffer containing the property data."] # [doc = ""] # [doc = " Returns: returns noErr if successful."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `in_component` must be a valid pointer."] # [doc = " - `in_property_data` must be a valid pointer."] # [cfg (feature = "AudioComponent")] pub fn AudioFileComponentSetProperty (in_component : AudioFileComponent , in_property_id : AudioFileComponentPropertyID , in_property_data_size : u32 , in_property_data : NonNull < c_void > ,) -> OSStatus ; }
};
}
