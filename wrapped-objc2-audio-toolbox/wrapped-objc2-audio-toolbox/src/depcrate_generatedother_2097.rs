// Generated macro for other_2097 (other)
macro_rules! Depcrate_generatedother_2097 {
() => {
// Module: crate::generated
// Provides: {"other_2097"}
// Dependencies: {}
extern "C-unwind" { # [doc = " implements AudioFileGetUserDataAtOffset."] # [doc = ""] # [doc = " Parameter `inComponent`: an AudioFileComponent"] # [doc = ""] # [doc = " Parameter `inUserDataID`: the four char code of the chunk."] # [doc = ""] # [doc = " Parameter `inIndex`: an index specifying which chunk if there are more than one."] # [doc = ""] # [doc = " Parameter `inOffset`: offset from the first byte of the chunk to the first byte to get."] # [doc = ""] # [doc = " Parameter `ioUserDataSize`: the size of the buffer on input, size of bytes copied to buffer on output"] # [doc = ""] # [doc = " Parameter `outUserData`: a pointer to a buffer in which to copy the chunk data."] # [doc = ""] # [doc = " Returns: returns noErr if successful."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `in_component` must be a valid pointer."] # [doc = " - `io_user_data_size` must be a valid pointer."] # [doc = " - `out_user_data` must be a valid pointer."] # [cfg (feature = "AudioComponent")] pub fn AudioFileComponentGetUserDataAtOffset (in_component : AudioFileComponent , in_user_data_id : u32 , in_index : u32 , in_offset : i64 , io_user_data_size : NonNull < u32 > , out_user_data : NonNull < c_void > ,) -> OSStatus ; }
};
}
