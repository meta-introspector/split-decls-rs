// Generated macro for other_2104 (other)
macro_rules! Depcrate_generatedother_2104 {
() => {
// Module: crate::generated
// Provides: {"other_2104"}
// Dependencies: {}
extern "C-unwind" { # [doc = " implements AudioFileGetGlobalInfoSize."] # [doc = ""] # [doc = " Parameter `inComponent`: an AudioFileComponent"] # [doc = ""] # [doc = " Parameter `inPropertyID`: an AudioFileGlobalInfo property constant."] # [doc = ""] # [doc = " Parameter `inSpecifierSize`: The size of the specifier data."] # [doc = ""] # [doc = " Parameter `inSpecifier`: A specifier is a buffer of data used as an input argument to some of the global info properties."] # [doc = ""] # [doc = " Parameter `outPropertySize`: the size in bytes of the current value of the property. In order to get the property value,"] # [doc = " you will need a buffer of this size."] # [doc = ""] # [doc = " Returns: returns noErr if successful."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `in_component` must be a valid pointer."] # [doc = " - `in_specifier` must be a valid pointer or null."] # [doc = " - `out_property_size` must be a valid pointer."] # [cfg (feature = "AudioComponent")] pub fn AudioFileComponentGetGlobalInfoSize (in_component : AudioFileComponent , in_property_id : AudioFileComponentPropertyID , in_specifier_size : u32 , in_specifier : * const c_void , out_property_size : NonNull < u32 > ,) -> OSStatus ; }
};
}
