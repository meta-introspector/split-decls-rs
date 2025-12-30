// Generated macro for other_2081 (other)
macro_rules! Depcrate_generatedother_2081 {
() => {
// Module: crate::generated
// Provides: {"other_2081"}
// Dependencies: {}
extern "C-unwind" { # [doc = " implements AudioFileOpenWithCallbacks"] # [doc = ""] # [doc = " Parameter `inComponent`: an AudioFileComponent"] # [doc = ""] # [doc = " Parameter `inClientData`: a constant that will be passed to your callbacks."] # [doc = ""] # [doc = " Parameter `inReadFunc`: a function that will be called when AudioFile needs to read data."] # [doc = ""] # [doc = " Parameter `inWriteFunc`: a function that will be called when AudioFile needs to write data."] # [doc = ""] # [doc = " Parameter `inGetSizeFunc`: a function that will be called when AudioFile needs to know the file size."] # [doc = ""] # [doc = " Parameter `inSetSizeFunc`: a function that will be called when AudioFile needs to set the file size."] # [doc = ""] # [doc = " Returns: returns noErr if successful."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `in_component` must be a valid pointer."] # [doc = " - `in_client_data` must be a valid pointer."] # [doc = " - `in_read_func` must be implemented correctly."] # [doc = " - `in_write_func` must be implemented correctly."] # [doc = " - `in_get_size_func` must be implemented correctly."] # [doc = " - `in_set_size_func` must be implemented correctly."] # [cfg (all (feature = "AudioComponent" , feature = "AudioFile"))] pub fn AudioFileComponentOpenWithCallbacks (in_component : AudioFileComponent , in_client_data : NonNull < c_void > , in_read_func : AudioFile_ReadProc , in_write_func : AudioFile_WriteProc , in_get_size_func : AudioFile_GetSizeProc , in_set_size_func : AudioFile_SetSizeProc ,) -> OSStatus ; }
};
}
