// Generated macro for AudioFileComponentWriteBytes (function)
macro_rules! Depcrate_generatedAudioFileComponentWriteBytes {
() => {
// Module: crate::generated
// Provides: {"AudioFileComponentWriteBytes"}
// Dependencies: {}
# [doc = " implements AudioFileWriteBytes."] # [doc = ""] # [doc = " Parameter `inComponent`: an AudioFileComponent"] # [doc = ""] # [doc = " Parameter `inUseCache`: true if it is desired to cache the data upon write, else false"] # [doc = ""] # [doc = " Parameter `inStartingByte`: the byte offset where the audio data should be written"] # [doc = ""] # [doc = " Parameter `ioNumBytes`: on input, the number of bytes to write, on output, the number of"] # [doc = " bytes actually written."] # [doc = ""] # [doc = " Parameter `inBuffer`: inBuffer should be a void * containing the bytes to be written"] # [doc = ""] # [doc = " Returns: returns noErr if successful."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `in_component` must be a valid pointer."] # [doc = " - `io_num_bytes` must be a valid pointer."] # [doc = " - `in_buffer` must be a valid pointer."] # [cfg (feature = "AudioComponent")] # [inline] pub unsafe extern "C-unwind" fn AudioFileComponentWriteBytes (in_component : AudioFileComponent , in_use_cache : bool , in_starting_byte : i64 , io_num_bytes : NonNull < u32 > , in_buffer : NonNull < c_void > ,) -> OSStatus { extern "C-unwind" { fn AudioFileComponentWriteBytes (in_component : AudioFileComponent , in_use_cache : Boolean , in_starting_byte : i64 , io_num_bytes : NonNull < u32 > , in_buffer : NonNull < c_void > ,) -> OSStatus ; } unsafe { AudioFileComponentWriteBytes (in_component , in_use_cache as _ , in_starting_byte , io_num_bytes , in_buffer ,) } }
};
}
