// Generated macro for other_2094 (other)
macro_rules! Depcrate_generatedother_2094 {
() => {
// Module: crate::generated
// Provides: {"other_2094"}
// Dependencies: {}
extern "C-unwind" { # [doc = " implements AudioFileGetUserDataSize"] # [doc = ""] # [doc = " Parameter `inComponent`: an AudioFileComponent"] # [doc = ""] # [doc = " Parameter `inUserDataID`: the four char code of the chunk."] # [doc = ""] # [doc = " Parameter `inIndex`: an index specifying which chunk if there are more than one."] # [doc = ""] # [doc = " Parameter `outUserDataSize`: on output, if successful, the size of the user data chunk."] # [doc = ""] # [doc = " Returns: returns noErr if successful."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `in_component` must be a valid pointer."] # [doc = " - `out_user_data_size` must be a valid pointer."] # [cfg (feature = "AudioComponent")] pub fn AudioFileComponentGetUserDataSize (in_component : AudioFileComponent , in_user_data_id : u32 , in_index : u32 , out_user_data_size : NonNull < u32 > ,) -> OSStatus ; }
};
}
