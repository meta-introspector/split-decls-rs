// Generated macro for other_2098 (other)
macro_rules! Depcrate_generatedother_2098 {
() => {
// Module: crate::generated
// Provides: {"other_2098"}
// Dependencies: {}
extern "C-unwind" { # [doc = " implements AudioFileSetUserData."] # [doc = ""] # [doc = " Parameter `inComponent`: an AudioFileComponent"] # [doc = ""] # [doc = " Parameter `inUserDataID`: the four char code of the chunk."] # [doc = ""] # [doc = " Parameter `inIndex`: an index specifying which chunk if there are more than one."] # [doc = ""] # [doc = " Parameter `inUserDataSize`: on input the size of the data to copy, on output, size of bytes copied from the buffer"] # [doc = ""] # [doc = " Parameter `inUserData`: a pointer to a buffer from which to copy the chunk data"] # [doc = " (only the contents of the chunk, not including the chunk header)."] # [doc = ""] # [doc = " Returns: returns noErr if successful."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `in_component` must be a valid pointer."] # [doc = " - `in_user_data` must be a valid pointer."] # [cfg (feature = "AudioComponent")] pub fn AudioFileComponentSetUserData (in_component : AudioFileComponent , in_user_data_id : u32 , in_index : u32 , in_user_data_size : u32 , in_user_data : NonNull < c_void > ,) -> OSStatus ; }
};
}
